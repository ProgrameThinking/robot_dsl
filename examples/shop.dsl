# Author: SakurakojiSaika

global name="";
global bill=0;

loop{
    speak "你好，"+name+"请问有什么需要帮助的？";
    speak "b：查看账户余额";
    speak "r：充值账户余额";
    speak "c：进行投诉";
    speak "e：退出程序";
    input str;
    if(str=="b") {
        speak "你的账户余额为"+bill;
    };
    if(str=="c") {
        speak "请输入您的建议。";
        input x;
        speak "感谢您的投诉";
    };
    if(str=="r") {
        speak "请输入充值的金额:";
        input x;
        bill=bill+x;
        speak "充值成功!";
    };
    if(str=="e") {
        exit;
    };
}
