function main(){
   
    let a = 233 + 2323;
    let str = JSON.stringify({a: 1, b: 2});
    let r =  recursive(4);
    return {a:1, b:2};

}

function recursive(fact){
    if(fact === 1){
        return 1;
    }
    return fact * recursive(fact - 1);
}
main();