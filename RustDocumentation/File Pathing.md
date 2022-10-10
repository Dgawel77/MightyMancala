# Intro
file pathing is  dependant on the current working directory. 
For experementing you can use 
```
std::env::current_dir()
```
to get a PathBuf to the cwd

Basically the directory you run cargo run from will be the cwd. This mean that if you have relative paths then they will change if the directory you run cargo run in changes. 