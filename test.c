int m(int a,int b){
  int c = 100+a;
  if(c==100){
    a=10;
    if(b==2000){
      c=100;
      return a;
    }
  } else{
    a=20;
  }
  return a+1*b+(b*1111+2);
}