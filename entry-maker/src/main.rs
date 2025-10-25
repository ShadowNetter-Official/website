use std::{
    fs,
    process,
};
use prompted::input;

fn main() {
    let (title, subtitle, github_link, image_link, langs, description) = get_input();
    let html = write(&title, &subtitle, &langs, &image_link, &github_link, &description);
    println!("added html: \n{html}\n");
    preview(&title, &subtitle, &langs, &description, &image_link, &github_link);
}

fn get_input() -> (String, String, String, String, String, String) {
    let title = input!("Project name: ");
    let subtitle = input!("Project description (short): ");
    let github_link = input!("Project GitHub link: ");
    let image_link= input!("Image link: ");
    let langs = input!("Project languages: ");
    let description = input!("Project description (long): ");
    return (title, subtitle, github_link, image_link, langs, description);
}

fn preview(title: &String, subtitle: &String, langs: &String, description: &String, image_link: &String, github_link: &String) {
    println!("\nPreview: \n");
    println!("{title} ({github_link})");
    println!("{subtitle}\n");
    println!("Image ({image_link})");
    println!("Using: {langs}\n");
    println!("{description}\n");
}


fn write(title: &String, subtitle: &String, langs: &String, image_link: &String, github_link: &String, description: &String) -> String {
    let file = "projects.html";
    let mut contents = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(e) => {
            println!("error reading {file}: {e}");
            quit();
            return Default::default();
        },
    };
    let new_blog = format!(r#"
        <div class="project">
            <a href="{github_link}" target="_blank">
                <h2>{title}</h2>
            </a>
            <h3>{subtitle}</h3>
            <img src="{image_link}" alt="{title}">
            <h3>Using: {langs}</h3>
            <br>
            <p>
                {description}
            </p>
        </div>
        <!-- End -->
    "#);
    if let Some(pos) = contents.rfind("<!-- End -->") {
        contents.insert_str(pos, &new_blog);
    } else {
        println!("didn't find <!-- End --> element, add it to file then try again");
        quit();
    }

    match fs::write(&file, contents) {
        Ok(_) => (),
        Err(e) => {
            println!("error writing changes to {file}: {e}");
            quit();
        },
    }
    new_blog.to_string()
}

fn quit() {
    process::exit(0);
}
