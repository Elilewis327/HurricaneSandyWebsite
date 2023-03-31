function no_scroll(a){
    element = document.getElementById("main");
    if (a){
        element.classList.add("noscroll");
    } else {
        element.classList.remove("noscroll");
    }
}
