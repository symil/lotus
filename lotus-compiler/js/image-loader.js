export class ImageLoader {
    constructor() {
        this._images = {};
        this._loadingCount = 0;
    }

    get(url) {
        if (!url) {
            return null;
        }

        let image = this._images[url];
        
        if (image === undefined) {
            image = this._load(url);
        }

        return image;
    }

    isLoading() {
        return this._loadingCount > 0;
    }

    register(url, image) {
        this._images[url] = image;
    }

    _load(url) {
        let result = null;

        this._loadingCount += 1;
        loadImage(url).then(image => {
            this._images[url] = image;
            this._loadingCount -= 1;
        });

        this._images[url] = result;

        return result;
    }
}

function loadImage(url) {
    return new Promise(resolve => {
        let image = new Image();
        image.src = url;
        image.onload = () => {
            resolve(image);
        }
    });
}