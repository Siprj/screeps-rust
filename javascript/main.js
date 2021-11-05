let wasm = require('screeps-rust')
wasm.initialize_instance()

function loop ()
{
    wasm.__wasm.main();
}

module.exports.loop =  loop;

