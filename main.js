import express from "npm:express@4";


const app = express();

const index = await Deno.readTextFile("index.html");

const map1 = new Map();

app.get("/", (_, res) => {
    //const index = Deno.readTextFileSync("index.html");
    

    res.set('Content-Type', 'text/html');
    res.send(index);
});


app.use(express.json());


app.post("/new", (request, response) => {
    const info = request.body;
    
    if (!isJsonString(info)){
        response.send("Invalid json");
        return
    }

    let stringOk = false;
    let str = ""
    while (!stringOk){
        const strt = genRandonString(4);
        if (!map1.has(strt)){
            
            stringOk = true;
            str = strt;
        }else{
            const date = map1.get(strt+"_date");

            if (Date.now()-date >= 60000){
                
                map1.delete(strt);
                map1.delete(strt+"_date");

                stringOk = true;
                str = strt;

            }
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
        
        map1.delete(st);
        map1.delete(st+"_date");
        response.send(JSON.stringify({
            "text": "TooOLD"
        }));
        return

    }
    map1.delete(st);
    map1.delete(st+"_date");
    
    response.send(s);

});




app.listen({ hostname: "0.0.0.0", port: 3000 });


function genRandonString(length) {
    const chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    const charLength = chars.length;
    let result = '';
    for (let i = 0; i < length; i++ ) {
       result += chars.charAt(Math.floor(Math.random() * charLength));
    }
    return result;
 }
 function isJsonString(str) {
    try {
        JSON.parse(str);
    } catch (e) {
        return false;
    }
    return true;
}