# Fibbot GitHub Action

A GitHub Action written in Rust that scans pull request content for numbers, calculates their Fibonacci values, and posts the results as a comment.  

## Features  (What the Bot is all about)
- Extracts numbers from pull requests  
- Computes Fibonacci numbers  
- Posts results as a comment  
- Configurable with a flag and threshold  



### That which must be installed in your machine 
- Rust 
- Docker (for building and testing)  

### Build the Action  
```sh
git clone https://github.com/chojuninengu/FibBot.git  
cd FibBot  
cargo build --release  
docker build -t FibBot.
```
```sh
docker run --rm \
  -e GITHUB_TOKEN=your_token \
  -e GITHUB_REPOSITORY=owner/repo \
  -e PR_NUMBER=123 \
  fibbot --enable-fib --max-threshold 100
```


