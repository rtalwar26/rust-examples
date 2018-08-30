"use strict";
const addon = require('../build/Release/n-api-hello-native');
;
class NApiHello {
    constructor(name) {
        this._addonInstance = new addon.NApiHello(name);
    }
    greet(strName) {
        return this._addonInstance.greet(strName);
    }
}
module.exports = NApiHello;
