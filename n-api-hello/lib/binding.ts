const addon = require('../build/Release/n-api-hello-native');

interface INApiHelloNative
{
    greet(strName: string): string;
};

class NApiHello {
    constructor(name: string) {
        this._addonInstance = new addon.NApiHello(name)
    }

    greet (strName: string) {
        return this._addonInstance.greet(strName);
    }

    // private members
    private _addonInstance: INApiHelloNative;
}

export = NApiHello;
