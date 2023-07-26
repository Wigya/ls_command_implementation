# What is it?
  This is my pure rust implementation of unix's terminal command ```ls``` which allows user to list directories' contents. It supports 2 flags ```-h``` and ```-l```

# How to use it?
  ```cargo run -q ls [path_to_directory] [OPTIONAL flag -h or -l]```
![Screenshot 2023-07-26 at 13 50 13](https://github.com/Wigya/ls_command_implementation/assets/59150755/db8959e8-86b7-4d2b-a820-62dc0a50052d)

![Screenshot 2023-07-26 at 13 50 59](https://github.com/Wigya/ls_command_implementation/assets/59150755/f2d133c3-8612-49d6-a627-524237b118d8)

# Limitations
  Can't execute ls command without specyfing ```[path_to_directory]```, there must be at least ```.```
