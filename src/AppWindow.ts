export class AppWindow {
  _worker?: Worker

  constructor(private _id, private _createWorker) {}

  async loadURL(url) {
    this._worker?.terminate()

    this._worker = this._createWorker() as Worker
    this._worker.postMessage({ windowId: this._id, url })
  }

  // TODO: dispatch beforeunload and optionally call destroy()
  // async close() {}

  // TODO: destroy immediately (incl native window? or app should do that?)
  // destroy() {}
}
