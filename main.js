import express from "npm:express@4";


const app = express();

const index = await Deno.readTextFile("index.html");

const map1 = new Map();

app.get("/", (_, res) => {
    //const index = Deno.readTextFileSync("index.html");
    res.set('Content-Type', 'text/html');
    res.send(index);
});
app.get("/page2", (_, res) => {
    const index = Deno.readTextFileSync("page2.html");
    res.set('Content-Type', 'text/html');
    res.send(index);
});

app.use(express.json());


app.post("/new", (request, response) => {
    const info = request.body;
   

    let stringOk = false;
    let str = ""
    while (!stringOk){
        const strt = genRandonString(4);
        if (!map1.has(strt)){
            stringOk = true;
            str = strt;
        }
    } 
    if (str == ""){
        response.send("err");
        return
    }
    map1.set(str,info);
    map1.set(str+"_date",Date.now());


    response.send(str);
});

app.param('token', function(req, _, next, token) {
    
  
    req.token = token;
    next();
  });

app.get("/ret/:token", (request, response) => {
    const st = request.token

    const s = map1.get(st);
    const date = map1.get(st+"_date");
    if (Date.now()-date >= 60000){
        response.send(JSON.stringify({
            "text": "TooOLD"
        }));
        map1.delete(st);
        map1.delete(st+"_date");

        return

    }
    map1.delete(st);
    map1.delete(st+"_date");
    
    response.send(s);

});



app.listen(8000);


function genRandonString(length) {
    const chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    const charLength = chars.length;
    let result = '';
    for (let i = 0; i < length; i++ ) {
       result += chars.charAt(Math.floor(Math.random() * charLength));
    }
    return result;
 }