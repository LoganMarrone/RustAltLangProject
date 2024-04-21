# Rust Alternate Language Project
Alternative Language Project that sifts through a csv file using an alternative language other than C/C++/Python. This one is primarily focused on Rust

# Questions
### Which programming language and version did you pick?
I chose the Language Rust and the version I picked is 1.77.2. What I believe to be the latest/most stable version.

### Why did you pick this programming language?
I have always wondered about Rust and, given this opportunity, I decided to push myself to learn a completely new language rather than try and seek a language I have already dabbled in.

### How your programming language chosen handles: object-oriented programming, file ingestion, conditional statements, assignment statements, loops, subprograms (functions/methods), unit testing and exception handling. If one or more of these are not supported by your programming language, indicate it as so. 
Rust, in essence, does not have traditional features for Object-oriented programming such as inheritance or classes but, through traits, can mimic such a feature. Rust can read/write files easily given the use of the standard (std) library. If/else is featured as well along with a match feature that does pattern recognition. Assigning statements is quite unique in Rust where we can use let to make a variable but all variables are fixed unless you use mut for mutability. Loops such as loop, while, and for are supported as well. Subprograms, such as functions and methods, are supported primarily due to fn being representative as a function but can also be implemented using impl. Using their standard library, std, allows the user to test the program (unit testing). Finally, Rust does not allow for traditional exceptions but can use an enum (enumeration) called Result that forces explicit error handling that should be tackled first.

### List out 3 libraries you used from your programming language (if applicable) and explain what they are, why you chose them and what you used them for.
I primarily utilized the std (Standard) library within Rust that follows similar if not the same layout as the C++ STD (standard) library.

# Checklist

* [x] Uses alternative language (Java, C#, etc)
* [X] Read CSV file
* [X] Cell Class
* [X] Assign columns with class attributes
* [X] Getter/Setter Methods
* [X] Data structure native to Language
* [X] Replace missing/"-" values with null
* [X] Convert data types
