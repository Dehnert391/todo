use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, Write};


///
/// zur Erstellung einer JSON-Datei hier wird ein bestimmter Typ einer Struct an die Datei gebunden.
pub struct FileHandler <T> {
    path: String,
    input_type: std::marker::PhantomData<T>,
}


impl <T: serde::Serialize + for<'de> serde::Deserialize<'de>> FileHandler<T> {
    /// Erstellt eine Datei mit dem übergebenen Pfad wenn diese nicht schon verhanden ist.
    /// Dann wird das Objekt mit dem Pfad und den Akzeptierten Datentyp zurück gegeben.
    pub fn build(path:&str) -> Self {

        if File::open(&path).is_err() {
            fs::File::create(&path).expect(format!("Fehler beim erstellen der Datei {:?}", &path).as_str());
        }  

        FileHandler{
            path:path.to_string(),
            input_type: std::marker::PhantomData,
        }
    }
    /// die Datei wird ausgelesen.
    pub fn read(&self) -> Result<Vec<T>, Box<dyn Error>> { 
        let data = File::open(&self.path)?;
        let reader = BufReader::new(data);
        let content:Vec<T> = serde_json::from_reader(reader)?; 
        Ok(content)
        }

    /// Die Datei wird geleert und dann geschrieben.
    pub fn write(&self, input: &Vec<T>) -> Result<(), Box<dyn std::error::Error>>{

        let mut file = OpenOptions::new()
                                        .truncate(true)
                                        .write(true)
                                        .open(&self.path)?;
                                        
        let write_in_file = serde_json::to_string_pretty(input)?;
        file.write_all(write_in_file.as_bytes())?;

        Ok(())
    }
}
