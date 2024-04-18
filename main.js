import express from "npm:express@4";


const app = express();

const index = await Deno.readTextFile("index.html");

const map1 = new Map();
const TIME = 60000;
let am_bots = 0;

app.get("/", (_, res) => {
    //const index = Deno.readTextFileSync("index.html");
    

    res.set('Content-Type', 'text/html');
    res.send(index);
});

app.get("/rb", (_, res) => {
    //const index = Deno.readTextFileSync("index.html");
    const index = ""
    am_bots += 1
    res.set('Content-Type', 'text/html');
    res.send(index);
});
app.get("/rbl", (_, res) => {
    //const index = Deno.readTextFileSync("index.html");
    const index = "am rec "+am_bots;
    
    res.set('Content-Type', 'text/html');
    res.send(index);
});



app.use(express.json());


app.post("/new", (request, response) => {
    const pinfo = JSON.stringify(request.body);
    
    if (!isJsonString(pinfo)){
        response.send("Invalid json");
        return
    }
    const info = JSON.parse(pinfo)

    let stringOk = false;
    let str = ""
    while (!stringOk){
        const strt = genRandonString(4);
        if (!map1.has(strt)){
            
            stringOk = true;
            str = strt;
        }/*else{
            const date = map1.get(strt+"_date");

            if (Date.now()-date >= TIME){
                
                map1.delete(strt);
                map1.delete(strt+"_date");

                stringOk = true;
                str = strt;

            }
        }*/
    } 
    if (str == ""){
        response.send("err");
        return
    }
    let save_time = parseInt(info.save_time);
    if (save_time > 10){
        save_time = 10;
    }
    if (save_time < 1){
        save_time = 1
    }

    map1.set(str,info);
    
   // map1.set(str+"_date",Date.now());
    setTimeout(()=>{
        map1.delete(str);
        map1.delete(str+"_date")
    }, save_time*TIME)

    response.send(str);
});

app.param('token', function(req, _, next, token) {
    
  
    req.token = token;
    next();
  });

app.get("/ret/:token", (request, response) => {
    const st = request.token

    if (!map1.has(st)){
        
      //  map1.delete(st);
       // map1.delete(st+"_date");
        response.send(JSON.stringify({
            "text": "The Token used has expired / You used the wrong token"
        }));
        return

    }
    const s = map1.get(st);
    
    
    map1.delete(st);
   // map1.delete(st+"_date");
    
    response.send(s);

});



const PORT = Deno.env.get("PORT");
app.listen({ hostname: "0.0.0.0", port: PORT });


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