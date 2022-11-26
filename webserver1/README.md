# Webserver - Learning from Scratch 2

- Client connected threads on thread pool 
- Each thread sends data to a database connected thread through mpsc.
- The database connected thread send result to the client connected threads through a customized reverse mpsc.
