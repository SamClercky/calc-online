export function getElementOffset(element: HTMLElement) {
    const x = window.scrollX + element.getBoundingClientRect().left // X
    const y = window.scrollY + element.getBoundingClientRect().top // Y

    return {
        "x": x,
        "y": y
    };
}