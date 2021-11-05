module.exports.kwa = function kwa() { return "bal"; }
module.exports.blakwa = function blakwa() { return "blakwa"; }
module.exports.log_keys = function log_keys(v) {
    for(const i in v) {
        console.log(i);
    }
}

module.exports.obj_map = function (obj, f) {
    for(var k in obj) {
        f(obj[k]);
    }
}
