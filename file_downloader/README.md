## File Downloader

This project allows you to download a file through a url 

### Feature

- It uses `request` and standard library fs to download the files

### How to run 

To run it locally you need to pass two arguments the first being the link and the second being the filename with the same extension as the remote file your are trying to download.

```sh
cargo run "https://www-verimag.imag.fr/\~mounier/Enseignement/Software_Security/19RustVsC.pdf" "rust_file.pdf"                            
```