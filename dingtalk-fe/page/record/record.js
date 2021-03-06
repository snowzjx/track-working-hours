let app = getApp();

let domain = "http://ec2-52-81-189-87.cn-north-1.compute.amazonaws.com.cn:3000";
let track_url = domain + '/dingtalk/record'

Page({
  data: {
    recordedTrackings: [],
  },
  loadTrackings() {
    dd.showLoading();
    dd.httpRequest({
      url: track_url,
      method: 'GET',
      dataType: 'json',
      success: (res) => {
        this.setData({
          recordedTrackings: res.data,
        });
        console.log(res.data);
      },
      fail: (res) => {
        console.log("httpRequestFail---",res)
        dd.alert({content: JSON.stringify(res)});
      },
      complete: (res) => {
        dd.hideLoading();
      },
    });
  },
  onLoad() {
    let _this = this;
    this.loadTrackings()
  },
});
