const fs = require('fs');
const input = fs.readFileSync('/dev/stdin').toString().split(' ');
// let input = [100000000000,100001000000]
// let input = [15,15]
// let input = [1,1000]
let min = parseInt(input[0]);
let max = parseInt(input[1]);
// print(min, max)

let t1 = Date.now()
che = new Array(max-min+1).fill(1).map((v,i)=>i+min)// [1 for _ in range(max-min)]
p_list = [2,3,5,7,11,13,17,19,23,29]
for (let i=31; i<max**0.334; i+=2){
    f = true
    for (let p of p_list){
        if (i%p==0){
            f = false
            break
        }
    }
    if (f) p_list.push(i)
}
//console.log(p_list)
let t2 = Date.now()

for (p of p_list){
    P = p*p
    for (let i=(-Math.floor(-min/p))*p; i<=max; i+=p){
    //     che[i-min] = 0
    // for (let i=min; i<=max; i++) if(i){
        //if (i%p) continue;
        //else 
        if (i%P) che[i-min] /= p;
        else  che[i-min] = 0
    }
}

let t3 = Date.now()
// console.log('che',che)
sum = 0
for (let n of che) if(n) {
    if (n==1) sum++
    else if(n**0.5  != parseInt(n**0.5)) sum++
}

// 이 수가 2*3*p*p면?
// 극단적으로 p*p*p면?

// sum = 0
// for(let i=min; i<=max; i++){
//     let n = i
//     let flag = true //제곱수 아닌게 사실임
//     for (let p of p_list) {
//         const P = p*p;
//         if (n%p) continue
//         else if(n%P==0){
//             flag = false;
//             break;
//         }else if(n==p){
//             break
//         }else{
//             n=parseInt(n/p)
//         }
//     }
//     if (flag){
//         //n은  세 제곱근 보다 큰 소수들의 곱 ppp, ppq, pqr 불가능
//         // ex p, pp, pq, 
//         //console.log('flag last',n,i)
//         if (n==1) sum++;
//         else if(n**0.5  != parseInt(n**0.5)) sum++
        
//     }
// }
// for(let f of che) sum+=f

let t4 = Date.now()
console.log(sum)
// console.log(t2-t1,t3-t2,t4-t3)