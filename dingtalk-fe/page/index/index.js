let app = getApp();


//内网穿透工具介绍:
// https://open-doc.dingtalk.com/microapp/debug/ucof2g
//替换成开发者后台设置的安全域名
let domain = "http://ec2-52-81-189-87.cn-north-1.compute.amazonaws.com.cn:3000";
let login_url = domain + '/dingtalk/login';
// let url = domain + '/login';
let assigned_project_url = domain + '/dingtalk/assigned_projects';
let record_form_url = domain + '/dingtalk/record/add';

Page({
    data:{
        corpId: '',
        authCode: '',
        userId: '',
        userName:'',
        assignedProjects: [],
        hideList: true,
    },
    loginSystem() {
        dd.showLoading();
        dd.getAuthCode({
            success:(res)=>{
                this.setData({
                    authCode:res.authCode
                })
                //dd.alert({content: "step1"});
                dd.httpRequest({
                    url: login_url,
                    method: 'POST',
                    data: {
                        authCode: res.authCode
                    },
                    dataType: 'json',
                    success: (res) => {
                        // dd.alert({content: "step2"});
                        console.log('success----',res)
                        let userId = res.data.userId;
                        let userName = res.data.userName;
                        this.setData({
                            userId:userId,
                            userName:userName,
                            hideList:false
                        })
                    },
                    fail: (res) => {
                        console.log("httpRequestFail---",res)
                        dd.alert({content: JSON.stringify(res)});
                    },
                    complete: (res) => {
                        dd.hideLoading();
                        this.loadProject();
                    }
                    
                });
            },
            fail: (err)=>{
                // dd.alert({content: "step3"});
                dd.alert({
                    content: JSON.stringify(err)
                })
            }
        })

    },
    loadProject() {
        dd.showLoading();
        dd.httpRequest({
            url: assigned_project_url,
            method: 'GET',
            dataType: 'json',
            success: (res) => {
                console.log('success----',res)
                this.setData ({
                    assignedProjects: res.data
                });
            },
            fail: (err)=>{
                // dd.alert({content: "step3"});
                dd.alert({
                    content: JSON.stringify(err)
                })
            },
            complete: (res) => {
                dd.hideLoading();
            }
        });
    },

    formSubmit: function(e)  {
        console.log(e.detail.value);
        dd.httpRequest({
            url: record_form_url,
            method: 'POST',
            data: e.detail.value,
            dataType: 'json',
            success: (res) => {
                dd.navigateTo({url: "/page/record/record"});  
            },
            fail: (res) => {
                console.log("httpRequestFail---",res)
                dd.alert({content: JSON.stringify(res)});
            },
            complete: (res) => {
                dd.hideLoading();
            }
        });
    },
    
    onLoad() {

        let _this = this;

        this.setData({
            corpId: app.globalData.corpId
        })

        this.loginSystem();

        //dd.alert({content: "step1"});
        
    }
})