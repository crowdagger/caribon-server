function on(name) {
        var elements = document.getElementsByClassName(name);
        for (var i = 0; i < elements.length; i++) {
            var elem = elements[i];
            elem.style.backgroundColor = "pink";
        }
}
function off(name) {
    var elements = document.getElementsByClassName(name);
    for (var i = 0; i < elements.length; i++) {
        var elem = elements[i];
        elem.style.backgroundColor = "white";
    }
}

function load_result()
{
    document.getElementById("result").innerHTML="<p>Computing result...</p>";
    var editable = document.getElementById("editable");
    var text_input = document.getElementById("text_input");
    text_input.value = editable.innerHTML;
    var element = document.getElementById("caribon_form");
    console.log("element:"+element);
    var xmlhttp = new XMLHttpRequest();
    xmlhttp.onreadystatechange = function () {
        if ( 4 != xmlhttp.readyState ) {
            return;
        }
        if (200 != xmlhttp.status) {
            document.getElementById("result").innerHTML="<span class = 'alert label'>" +
                xmlhttp.responsetext +
                "<span>";
            return;
        }
        document.getElementById("result").innerHTML=xmlhttp.responseText;
    }
    console.log("action:" + element.action);
    xmlhttp.open(element.method, element.action, true);
    console.log("element:" + element);
    var data = serialize(element);
    console.log("data" + data);
    xmlhttp.setRequestHeader('Content-Type','application/x-www-form-urlencoded')
    xmlhttp.timeout = "4000";
    xmlhttp.ontimeout = function() {
        document.getElementById("result").innerHTML="<span class = 'alert label'>Server took too much time to answer, aborting!</span>";
    };
    xmlhttp.send(data);
    return false;
}





