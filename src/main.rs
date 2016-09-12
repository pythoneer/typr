#![allow(non_snake_case)]
#[macro_use]

extern crate qml;

use qml::*;


#[derive(Debug)]
pub struct ContextModel {
    list_of_words: Vec<String>,
    list_of_validations: Vec<bool>,
    current_context_word: String,
}

impl ContextModel {
    fn context_to_string(&self) -> String {
        let mut output = "".to_string();
        let valid_length = self.list_of_validations.len();

        for (idx, word) in self.list_of_words.iter().enumerate() {
            
            let mut formatted: String;

            if(valid_length < idx) {
                formatted = format!("<font  color=\"black\">{}</font> ", word);
            } else if(valid_length == idx) {
                formatted = format!("<font  color=\"blue\">{}</font> ", word);
            } else {
                if(self.list_of_validations[idx]) {
                    formatted = format!("<font  color=\"green\">{}</font> ", word);
                } else {
                    formatted = format!("<font  color=\"red\">{}</font> ", word);    
                }
            }

            output.push_str(&formatted);
        }

        output
    }

    fn advance(&mut self) -> bool {
        let position = self.list_of_validations.len();
        if(position < self.list_of_words.len()) {
            let new_word = self.list_of_words[position].clone();
            self.current_context_word = new_word;
            return true;
        } 

        false
    }
}

pub struct WordModel {
    context_model: ContextModel
}

Q_OBJECT!(
    pub WordModel as QWordModel {
    signals:
        fn update();
        fn emptyInput();
    slots:
        fn validate();
        fn guiReady();
    properties:
        current_word: String; read: get_current_word, write: set_current_word, notify: current_word_changed;
        current_context: String; read: get_current_context, write: set_current_context, notify: current_context_changed;
    }
);

//impl slots
impl QWordModel {

    pub fn guiReady(&mut self) -> Option<&QVariant> {
        println!("GUI IS READY");

        let new_context = self.context_model.context_to_string();
        self.set_current_context(new_context);
        self.context_model.advance();

        self.update();
        None
    }

    pub fn validate(&mut self) -> Option<&QVariant> {
        //println!("validating");
         
        let current_word = String::from(self.get_current_word());

        //println!("current word: {:?}", current_word);

        if current_word.len() != current_word.trim().len() {
            //println!("SPACE");
            self.emptyInput();

            {
                let context_word = self.context_model.current_context_word.clone();

                println!("{:?}, {:?}", context_word, current_word.trim());

                if(context_word == current_word.trim()) {
                    println!("WORDS ARE THE SAME");
                    self.context_model.list_of_validations.push(true);
                } else {
                    println!("words are not the same");
                    self.context_model.list_of_validations.push(false);
                }
            }
             
            let is_more = self.context_model.advance();

            println!("is more: {:?}", is_more);

            let new_context = self.context_model.context_to_string();
            self.set_current_context(new_context);
        }

        self.update();

        None
    }

}



fn main() {
    let mut qEngine = QmlEngine::new();

    let mut contextModel = ContextModel{
        list_of_words: vec!["das".to_string(), "ist".to_string(), "ein".to_string(), "test".to_string(), "hoffe".to_string(), "ich".to_string(), "mal".to_string(), "sehr".to_string(), ],
        list_of_validations: vec![], 
        current_context_word: "".to_string()
    };

    let mut word_model = QWordModel::new(WordModel{context_model: contextModel}, "empty".to_string(), "space um zu starten".to_string());

    qEngine.set_and_store_property("WordModel", word_model.get_qobj());

    qEngine.load_file("qml/ui/main.qml");
    qEngine.exec();
    qEngine.quit();
}
