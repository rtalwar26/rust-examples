declare class NApiHello {
    constructor(name: string);
    greet(strName: string): string;
    private _addonInstance;
}
export = NApiHello;
