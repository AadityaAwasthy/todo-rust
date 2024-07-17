# CLI Todo List in Rust

## About
This is my first personal project, implemented from scratch in Rust. The main aim of this project was to create a simple cli application that uses CRUD operation on a database with robust error handling and designed in a way that it can be further extended and reused to add features.

With this todo list manager you can add tasks, delete tasks, edit the descriptions of tasks and change the status of tasks from pending to done. The application provides simple commands to help users with these tasks.

## Commands 
'''
structure of query: 
    command [arguments] 

supported commands: 
    add - Add a new task to the todo list, followed by a new task string. The task string should NOT be space separated. 

        usage: >add task_string

    list - Display the todo list 
        
        usage: >list

    delete - delete a task from the todo list, based on the task id provided by the user in the prompt. 

        usage: >delete task_id

    update - change the name of a task, followed by an integer number task id. 

        usage: >update task_id new_task_string 

    done - change the done status of a task from false to true, followed by an integer number task id. 
        
        usage: >done task_id 

    exit- exit the program. 
        
        usage: >exit

    help - display this help message. 
        
        usage: >help 

arguments: 
    task_id: the unique id assigned to each task. 

    task_string: the string for the task provided by the user. ";

'''
