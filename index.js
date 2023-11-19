const rusty_package = require('wasm-pack-hello-world')

function main(){
    const person = rusty_package.create_person("Hans", "Fiesel", 60);
    console.log(person);
}

main()