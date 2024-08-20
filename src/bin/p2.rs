use std::path::PathBuf;
use structopt::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "Contact Manager", about = "Simple app for contact manager.")]
struct Opt{
    #[structopt(short = "v", long = "view")]
    view: bool,

    #[structopt(parse(from_os_str))]
    contacts_file: PathBuf,
}

fn main() {
    let opts: Opt = Opt::from_args();

    println!("{:?}", opts);
}

struct Contact{
    id: i32,
    name: String,
    email: String,
}

impl Contact{
    fn view_contact(contact: &Self){
        Self::display_id(&contact);
        Self::display_name(&contact);

        if contact.email.is_empty(){
            Self::display_email(&contact);
        }
    }

    fn display_id(contact: &Self) {
        println!("Id: {}", contact.id.to_string())
    }

    fn display_name(contact: &Self){
        println!("Name: {}", contact.name.to_owned())
    }

    fn display_email(contact: &Self){
        println!("E-Mail: {}", contact.email.to_owned())
    }
}