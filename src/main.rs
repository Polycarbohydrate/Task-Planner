// Used for creation and manipulation of files and
// directories for storage of tasks and configuration
use std::fs::{create_dir, File, OpenOptions};
use std::io::{stdin, Read, Seek, SeekFrom, Write};
use std::path::Path;

// To check username to create the directories and files
use std::env;

// Structure for the tasks
struct Task<T> {
    task_name: T,
    task_description: T,
    task_date: T,
    task_time: T,
    task_priority: T,
}

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Task<String> {
    fn describe(&self) -> String {
        format!(
            "Task: {}\nDescription: {}\nDate: {}\nTime: {}\nPriority: {}\n\n",
            self.task_name,
            self.task_description,
            self.task_date,
            self.task_time,
            self.task_priority
    }
}

// Function to get user inputs and reduce redundancy
fn inputs() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_uppercase().to_string()
}

// Function to get the username of the user and reduce redundancy
fn get_username() -> String {
    env::var("USERNAME").expect("Failed to get username")
}

// Start off with checking of the configuration file
fn main() {
    config_file();
}

// Check the configuration file
fn config_file() {
    // Check the username of the user to create the directories and files
    let path_of_txt = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner\\Config.txt",
        get_username()
    );
    let path_of_dir = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner",
        get_username()
    );

    // Check if the file exists and read the contents
    // If the file does not exist, create the file
    // If the file exists, check the contents and act accordingly
    let hypothetical_file = Path::new(path_of_txt.as_str());
    if hypothetical_file.exists() {
        let mut check_returning_value =
            File::open(path_of_txt.as_str()).expect("Failed to open file");
        let mut contents = String::new();
        check_returning_value
            .read_to_string(&mut contents)
            .expect("Failed to read file");
        if contents.trim() == "Returning: 0" {
            check_tasks_file();
            first_disclaimer();
        } else if contents.trim() == "Returning: 1" {
            check_tasks_file();
            main_console();
        } else {
            first_disclaimer();
        }
    } else {
        create_dir(path_of_dir.as_str()).expect("Failed to create directory");
        let mut config_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path_of_txt)
            .expect("Failed to open file");
        config_file
            .write_all(b"Returning: 0")
            .expect("Failed to write to file");
        check_tasks_file();
        first_disclaimer();
    }
}

// Check the tasks file exists, otherwise create one
fn check_tasks_file() {
    // Check the username of the user to create the file
    let username = env::var("USERNAME").expect("Failed to get username");
    let path_of_txt = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner\\Tasks.txt",
        username
    );
    let hypothetical_file = Path::new(path_of_txt.as_str());
    if hypothetical_file.exists() {
    } else {
        let mut tasks_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path_of_txt)
            .expect("Failed to open file");
        tasks_file.write_all(b"").expect("Failed to write to file");
    }
}

// Display the first disclaimer for first time users
fn first_disclaimer() {
    // Check the username of the user to edit the config file
    let path_of_txt = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner\\Config.txt",
        get_username()
    );

    println!("--------------- Task Planner Program ---------------");
    println!("Copyright (c) 2025 Polycarbohydrate");
    println!("This program will write your scheduled tasks to a file");
    println!("in order to store them for later use. No data will be ");
    println!("collected, sent, or given to any third party or the");
    println!("developer of this program. Additionally, this program ");
    println!("will send desktop notifications to remind you of your ");
    println!("tasks. This program is not responsible for any missed ");
    println!("tasks or any other issues that may arise from using ");
    println!("this program. By using this program, you agree to the ");
    println!("terms and conditions of this program. If you do not ");
    println!("agree to these terms, please exit the program now. ");
    println!("To view the full list of what this program does, visit");
    println!("https://github.com/Polycarbohydrate/Task-Planner/blob/main/Privacy-Policy.md");
    println!(" ");
    println!("If you are uncomfortable with this program you can ");
    println!("exit now or any time by pressing 'Ctrl + C'.");
    println!(" ");
    println!("Would you like to continue? (y/n) (If you decide to ");
    println!("continue, this prompt will not appear again. If you ");
    println!("choose no, the program will close.");
    println!("-------------------------------------------------------");
    // If user decides to continue, change the configuration file to returning 1 and go to main console
    // If user decides to not continue, close the program and keep returning 0
    loop {
        let response = inputs();
        if response.trim().to_lowercase() == "y" {
            let mut change_config_file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .truncate(true)
                .open(path_of_txt)
                .expect("Failed to open file");
            let mut contents = String::new();
            change_config_file
                .read_to_string(&mut contents)
                .expect("Failed to read file");
            change_config_file
                .seek(SeekFrom::Start(0))
                .expect("Failed to seek file");
            change_config_file
                .set_len(0)
                .expect("Failed to truncate file");
            let new_contents = contents.replace("Returning: 0", "Returning: 1");
            change_config_file
                .write_all(new_contents.as_bytes())
                .expect("Failed to write to file");
            main_console();
            break;
        } else if response.trim().to_lowercase() == "n" {
            println!("Closing program...");
            break;
        } else {
            println!("----------------------------------------------------");
            println!("Invalid response. Please try again. (type 'y' or 'n'.");
            println!("Would you like to continue? (y/n) (If you decide to ");
            println!("continue, this prompt will not appear again. If you ");
            println!("choose no, the program will close.");
            println!("----------------------------------------------------");
            continue;
        }
    }
}

// Main console for the program for returning users and choosing what to do next
fn main_console() {
    println!("--------------- Task Planner Program ---------------");
    println!("Type in the option you would like to choose:");
    println!("1. Add a task");
    println!("2. View all tasks");
    println!("3. Remove a task");
    println!("4. Edit a task");
    println!("5. Mark a task as completed");
    println!("6. Exit the program");
    println!("7. Remove the program and its data/files");
    println!("8. Additional Options");
    println!("-------------------------------------------------------");
    loop {
        let decision = inputs();
        match decision.trim() {
            "1" => {
                add_task();
                break;
            }
            "2" => {
                view_tasks();
                break;
            }
            "3" => {
                remove_task();
                break;
            }
            "4" => {
                edit_task();
                break;
            }
            "5" => {
                mark_as_completed();
                break;
            }
            "6" => {
                println!("Closing program...");
                break;
            }
            "7" => {
                uninstall();
                break;
            }
            "8" => {
                additional_options();
                break;
            }
            _ => {
                println!(
                    "--------------------------------------------------------------------------"
                );
                println!(
                    "Invalid response. Please try again. (type '1', '2', '3', '4', '5', or '6'."
                );
                println!(
                    "--------------------------------------------------------------------------"
                );
                continue;
            }
        }
    }
}

fn add_task() {
    let task_file_path = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner\\Tasks.txt",
        get_username()
    );
    println!("--------------- Task Planner Program ---------------");
    println!("Please enter the name of the task:");
    let task_name = inputs();
    println!("Please enter the description of the task:");
    let task_description = inputs();
    println!("Please enter the date of the task (MM/DD/YYYY):");
    let task_date = inputs();
    println!("Please enter the time of the task (HH:MM AM/PM):");
    let task_time = inputs();
    println!("Please enter the priority of the task (High, Medium, Low):");
    let task_priority = inputs();
    let new_task = Task {
        task_name,
        task_description,
        task_date,
        task_time,
        task_priority,
    };
    let task_description = new_task.describe();
    let mut write_to_file = OpenOptions::new()
        .append(true)
        .open(task_file_path)
        .expect("Failed to open file");
    write_to_file
        .write_all(task_description.as_bytes())
        .expect("Failed to write to file");
    println!("----------------------------------------------------");
    println!("Task added");
    println!("Would you like to add another task? (y/n)");
    println!("----------------------------------------------------");
    loop {
        let response = inputs();
        if response.trim().to_lowercase() == "y" {
            add_task();
            break;
        } else if response.trim().to_lowercase() == "n" {
            main_console();
            break;
        } else {
            println!("----------------------------------------------------");
            println!("Invalid response. Please try again. (type 'y' or 'n'.");
            println!("Would you like to add another task? (y/n)");
            println!("----------------------------------------------------");
            continue;
        }
    }
    println!("----------------------------------------------------");
}

fn view_tasks() {
    println!("--------------- Task Planner Program ---------------");
    let task_file_path = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner\\Tasks.txt",
        get_username()
    );
    let mut read_from_file = File::open(task_file_path).expect("Failed to open file");
    let mut contents = String::new();
    read_from_file
        .read_to_string(&mut contents)
        .expect("Failed to read file");
    println!("Tasks:");
    println!("\n{}", contents);
    println!("----------------------------------------------------");
    press_btn_continue::wait("Press any key to continue...").unwrap();
    println!(" ");
    main_console();
}

fn remove_task() {
    println!("--------------- Task Planner Program ---------------");
    println!("Please enter the name of the task you would like to remove:");
    let task_name = inputs();
    let task_file_path = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner\\Tasks.txt",
        get_username()
    );
    let mut read_from_file = File::open(&task_file_path).expect("Failed to open file");
    let mut contents = String::new();
    read_from_file
        .read_to_string(&mut contents)
        .expect("Failed to read file");
    let mut updated_contents = String::new();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if line.trim() == format!("Task: {}", task_name) {
            lines.next();
            lines.next();
            lines.next();
            lines.next();
        } else {
            updated_contents.push_str(line);
            updated_contents.push('\n');
        }
    }
    let mut write_to_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&task_file_path)
        .expect("Failed to open file");
    write_to_file
        .write_all(updated_contents.as_bytes())
        .expect("Failed to write to file");
    println!("----------------------------------------------------");
    println!("Task removed");
    main_console();
}

fn edit_task() {
    println!("--------------- Task Planner Program ---------------");
    println!("Please enter the name of the task you would like to edit:");
    let task_name = inputs();
    let task_file_path = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner\\Tasks.txt",
        get_username()
    );
    let mut read_from_file = File::open(&task_file_path).expect("Failed to open file");
    let mut contents = String::new();
    read_from_file
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut updated_contents = String::new();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if line.trim() == format!("Task: {}", task_name) {
            lines.next();
            lines.next();
            lines.next();
            lines.next();

            println!("Please enter the new name of the task:");
            let new_task_name = inputs();
            println!("Please enter the new description of the task:");
            let new_task_description = inputs();
            println!("Please enter the new date of the task (MM/DD/YYYY):");
            let new_task_date = inputs();
            println!("Please enter the new time of the task (HH:MM AM/PM):");
            let new_task_time = inputs();
            println!("Please enter the new priority of the task (High, Medium, Low):");
            let new_task_priority = inputs();

            updated_contents.push_str(&format!("Task: {}\n", new_task_name));
            updated_contents.push_str(&format!("Description: {}\n", new_task_description));
            updated_contents.push_str(&format!("Date: {}\n", new_task_date));
            updated_contents.push_str(&format!("Time: {}\n", new_task_time));
            updated_contents.push_str(&format!("Priority: {}\n", new_task_priority));
        } else {
            updated_contents.push_str(line);
            updated_contents.push('\n');
        }
    }

    let mut write_to_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&task_file_path)
        .expect("Failed to open file");
    write_to_file
        .write_all(updated_contents.as_bytes())
        .expect("Failed to write to file");

    println!("----------------------------------------------------");
    println!("Task edited");
    main_console();
}

fn mark_as_completed() {
    println!("--------------- Task Planner Program ---------------");
    println!("Please enter the name of the task you would like to mark as completed:");
    let task_name = inputs();
    println!("----------------------------------------------------");
    let task_file_path = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Task Planner\\Tasks.txt",
        get_username()
    );
    let mut read_from_file = File::open(&task_file_path).expect("Failed to open file");
    let mut contents = String::new();
    read_from_file
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut updated_contents = String::new();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].trim() == format!("Task: {}", task_name) {
            updated_contents.push_str("COMPLETED\n");
        }
        updated_contents.push_str(lines[i]);
        updated_contents.push('\n');
        i += 1;
    }

    let mut write_to_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&task_file_path)
        .expect("Failed to open file");
    write_to_file
        .write_all(updated_contents.as_bytes())
        .expect("Failed to write to file");

    println!("----------------------------------------------------");
    println!("Task marked as completed");
    main_console();
}
fn uninstall() {
    println!("--------------- Task Planner Program ---------------");
    println!("Are you sure you would like to remove the program and all of its data stored in its files?");
    println!("This action cannot be undone. (y/n)");
    println!("----------------------------------------------------");
    let response = inputs();
    if response.trim().to_lowercase() == "y" {
        let username = env::var("USERNAME").expect("Failed to get username");
        let path_of_dir2 = format!("C:\\Users\\{}\\AppData\\Roaming\\Task Planner", username);
        let path_of_dir = Path::new(path_of_dir2.as_str());
        std::fs::remove_dir_all(path_of_dir).expect("Failed to remove directory");
        println!("----------------------------------------------------");
        println!("Program and all of its data has been removed");
        println!("----------------------------------------------------");
    } else if response.trim().to_lowercase() == "n" {
        println!("----------------------------------------------------");
        println!("Program and all of its data has not been removed");
        println!("----------------------------------------------------");
    } else {
        println!("----------------------------------------------------");
        println!("Invalid response. Please try again. (type 'y' or 'n'.");
        println!("----------------------------------------------------");
    }
    println!("----------------------------------------------------");
    println!("This program's data has been removed. If you do not wish to use this program anymore you can delete this executable.");
    press_btn_continue::wait("Press any key to continue...").unwrap();
    println!(" ");
    main_console();
}

fn additional_options() {
    println!("-------------------------------------------------------");
    println!("Type in the option you would like to choose:");
    println!("1. Privacy Policy");
    println!("2. License");
    println!("3. Credits");
    println!("4. Report a bug/request a feature");
    println!("5. Source Code / Project Page");
    println!("6. Back to main console");
    println!("-------------------------------------------------------");

    loop {
        let decision = inputs();
        match decision.trim() {
            "1" => {
                println!("-------------------------------------------------------");
                println!(
                    r#"
# Privacy Policy

Effective Date: 01/10/2025 MM/DD/YYYY

Last modified on 01/10/2025 (MM/DD/YYYY)

Task Planner ("we," "our," or "us") is committed to protecting your privacy. This Privacy Policy outlines how we manage user information.

## No Data Collection

We do not collect, store, or process any personal or non-personal data from our users. This means:

No Personal Data: We do not ask for or store any personally identifiable information (such as names
,email addresses, or phone numbers).

No Usage Data: We do not track or log your activity within the application.

No Third-Party Sharing: Since we do not collect data, there is no information to share with third
parties.

## Local Storage

The app uses local storage (e.g., to save preferences or settings), this data remains on your device and
is not accessible to us. All data is stored in the `C:\Users\[USERNAME]\AppData\Roaming\Task Planner`
folder on Windows.

## Changes to This Privacy Policy

We reserve the right to update this Privacy Policy at any time. If changes are made, the revised policy
will be posted within the application, and the effective date will be updated accordingly.

**By using Task Planner, you acknowledge and agree to the terms of this Privacy Policy.**

https://github.com/Polycarbohydrate/Task-Planner/blob/main/Privacy-Policy.md

                        "#
                );
                println!("-------------------------------------------------------");
                press_btn_continue::wait("Press any key to continue...").unwrap();
                println!(" ");
                additional_options();
                break;
            }
            "2" => {
                println!("-------------------------------------------------------");
                println!(
                    r#"
MIT License

Copyright (c) 2025 Polycarbohydrate

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

https://github.com/Polycarbohydrate/Task-Planner/blob/main/LICENSE

                        "#
                );
                println!("-------------------------------------------------------");
                press_btn_continue::wait("Press any key to continue...").unwrap();
                println!(" ");
                additional_options();
                break;
            }
            "3" => {
                println!("-------------------------------------------------------");
                println!("Credits:");
                println!(" ");
                println!("This program was created by Polycarbohydrate.");
                println!("https://github.com/Polycarbohydrate");
                println!(" ");
                println!("Contributors:");
                println!("https://github.com/Polycarbohydrate/Task-Planner/graphs/contributors");
                println!(" ");
                println!("-------------------------------------------------------");
                press_btn_continue::wait("Press any key to continue...").unwrap();
                println!(" ");
                additional_options();
                break;
            }
            "4" => {
                println!("-------------------------------------------------------");
                println!("Please visit https://github.com/Polycarbohydrate/Task-Planner/issues to report a bug or request a feature.");
                println!("-------------------------------------------------------");
                press_btn_continue::wait("Press any key to continue...").unwrap();
                println!(" ");
                additional_options();
                break;
            }
            "5" => {
                println!("-------------------------------------------------------");
                println!("Please visit https://github.com/Polycarbohydrate/Task-Planner to view the source code and project page.");
                println!("-------------------------------------------------------");
                press_btn_continue::wait("Press any key to continue...").unwrap();
                additional_options();
                break;
            }
            "6" => {
                main_console();
                break;
            }
            _ => {
                println!("-------------------------------------------------------");
                println!("Invalid response. Please try again. (type '1', '2', '3', or '4'.");
                println!("-------------------------------------------------------");
                continue;
            }
        }
    }
}
