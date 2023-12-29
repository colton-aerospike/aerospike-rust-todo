# Basic Todo App
Learning Aerospike rust client along with basic rust syntax. Create, complete, delete, and list todos stored in Aerospike DB.

# Results
```bash
Wrote record ID 1 successfully
WARNING! ID 2 already exists!
WARNING! ID 3 already exists!
WARNING! ID 4 already exists!
ID: 3, Task: test the todo app, Completed: false
ID: 2, Task: make a todo app, Completed: true
ID: 5, Task: my new task, Completed: false
ID: 4, Task: profit, Completed: false
ID: 1, Task: learn rust, Completed: false
Deleted record ID 1
ID: 3, Task: test the todo app, Completed: false
ID: 2, Task: make a todo app, Completed: true
ID: 5, Task: my new task, Completed: false
ID: 4, Task: profit, Completed: false
Marked record ID 2 as completed successfully
WARNING! ID 5 already exists!
ID: 3, Task: test the todo app, Completed: false
ID: 2, Task: make a todo app, Completed: true
ID: 5, Task: my new task, Completed: false
ID: 4, Task: profit, Completed: false
```
