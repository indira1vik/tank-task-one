# Rust and Webassembly

## Steps used for creating this project
1. Create new rust project using `cargo new --lib <folder>`
2. Open project in vscode using `code <folder>`
3. Make changes in `Cargo.toml` and `lib.rs` files.
4. Create a shell file named `run.sh` which contains the following commands
```
wasm-pack build --target bundler
cd pkg
npm link
cd ..
mkdir site
cd site
npm link <folder>
```
5. Create 4 new files in `site` folder
```
package.json
webpack.config.js
index.js
index.html
```
6. Copy the contents from the website like [MDN Rust Tutorial](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm#making_our_package_available_to_npm)
7. Change the dependencies' name as folder name in `package.json` file and in `index.js` file change the folder name.
8. If already present in `site` folder 
```
npm install
npm run serve
```
Else
```
cd site
npm install
npm run serve
```
9. Open [Localhost Link](https://localhost:8080) for the website hosted.

## Further details
Now we can work with `lib.rs` `index.html` `index.js` to make the changes to the website.
For every change we make and want to see the output in the web then 

Move into the `site` folder and commands
```
wasm-pack build --target bundler
npm run serve
```

## Steps to be followed when cloned from git repository
1. Clone project : `git clone https://github.com/indira1vik/first-task-tanks.git`
2. Follow the steps above from `STEP NUMBER 4`.

#### Final Output looks like : 
![Sample Screenshot](./screenshots/Large-Cuboidal-Orange.jpg)

Thank You.