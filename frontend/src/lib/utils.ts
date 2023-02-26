export function getValueFromObservable(observable) {
    let temp;
    observable.subscribe((value) => temp = value);
    return temp;
}