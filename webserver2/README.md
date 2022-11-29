# Webserver - Learning from Scratch 2

- Client connected threads on thread pool 
    (https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch20-03-designing-the-interface.html) 
- Each thread sends data to a database connected thread through mpsc.
- The database connected thread send result to the client connected threads through a customized reverse mpsc.
