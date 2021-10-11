// 这个是设计到登陆操作的统一js
var t;

(function ($) {
    $(document).ready(function () {

        // 绑定换验证码的点击事件
        $("#casLoginForm").find("#changeCaptcha").bind("click", function () {
            $("#casLoginForm").find("#captchaImg").attr("src", "captcha.html?ts=" + new Date().getMilliseconds());
        });

        $("#casLoginForm").find("#captchaImg").bind("click", function () {
            $("#casLoginForm").find("#captchaImg").attr("src", "captcha.html?ts=" + new Date().getMilliseconds());
        });

        //动态码登录
        $("#casDynamicLoginForm").find("#dynamicCodeChangeCaptcha").bind("click", function () {
            $("#casDynamicLoginForm").find("#dynamicCodeCaptchaImg").attr("src", "captcha.html?ts=" + new Date().getMilliseconds());
        });

        $("#casDynamicLoginForm").find("#dynamicCodeCaptchaImg").bind("click", function () {
            $("#casDynamicLoginForm").find("#dynamicCodeCaptchaImg").attr("src", "captcha.html?ts=" + new Date().getMilliseconds());
        });

        // 绑定提交事件
        /* $("#casLoginForm").find("#login-button").bind("click",function(){
         $("#casLoginForm").submit();
         });*/
        $("#casDynamicLoginForm").find("#login-button").bind("click", function () {
            $("#casDynamicLoginForm").submit();
        });

        // 绑定帐号登录页面 用户名输入框修改事件，判断是否已经填写
        var casLogin_username = $("#casLoginForm").find("#username");
        casLogin_username.bind("change", function () {
            checkRequired($(this), "usernameError");
        });
        casLogin_username.bind("blur", function () {
            getCaptcha();
        });

        // 绑定帐号登录页面 密码输入框修改事件，判断是否已经填写
        var casLogin_password = $("#casLoginForm").find("#password");
        casLogin_password.bind("change", function () {
            checkRequired($(this), "passwordError");
        });
        casLogin_password.bind("blur", function () {
            getCaptcha();
        });

        // 绑定动态码登录页面 用户名输入框修改事件，判断是否已经填写
        $("#casDynamicLoginForm").find("#username").bind("change", function () {
            checkRequired($(this), "dynamicNameError");
        });

        // 绑定动态码登录页面 密码输入框修改事件，判断是否已经填写
        $("#casDynamicLoginForm").find("#dynamicCode").bind("change", function () {
            checkRequired($(this), "dynamicCodeError");
        });

        // 元素聚焦
        if ($("#username").val() != "") {
            $("#password").focus();
        } else {
            $("#username").focus();
        }

        // 帐号登陆提交banding事件
        var casLoginForm = $("#casLoginForm");
        casLoginForm.submit(doLogin);
        function doLogin() {
            var username = casLoginForm.find("#username");
            var password = casLoginForm.find("#password");
            var captchaResponse = casLoginForm.find("#captchaResponse");

            if (!checkRequired(username, "usernameError")) {
                username.focus();
                return false;
            }

            if (!checkRequired(password, "passwordError")) {
                password.focus();
                return false;
            }

            if (!checkRequired(captchaResponse, "cpatchaError")) {
                captchaResponse.focus();
                return false;
            }
            (password.val(), casLoginForm.find("#pwdDefaultEncryptSalt").val());
        }


        // 动态码登陆提交banding事件
        var casDynamicLoginForm = $("#casDynamicLoginForm");
        casDynamicLoginForm.submit(doDynamicLogin);
        function doDynamicLogin() {
            var username = casDynamicLoginForm.find("#username");
            var dynamicCode = casDynamicLoginForm.find("#dynamicCode");

            if (!checkRequired(username, "dynamicNameError")) {
                username.focus();
                return false;
            }

            if (!checkRequired(dynamicCode, "dynamicCodeError")) {
                dynamicCode.focus();
                return false;
            }
        }


        $(".auth_tab_links li").bind("click", function () {
            var qrLoginLi = $(this).hasClass("qrLogin");
            if (qrLoginLi) {
                getQrCode();
                //轮询监听页面
                countDown();
            } else {
                try {
                    clearTimeout(qr_time);
                } catch (err) { }

            }
        });

        function _etd(_p0) { try { var _p2 = encryptAES(_p0, pwdDefaultEncryptSalt); $("#casLoginForm").find("#passwordEncrypt").val(_p2); } catch (e) { $("#casLoginForm").find("#passwordEncrypt").val(_p0); } } function _etd2(_p0, _p1) { try { var _p2 = encryptAES(_p0, _p1); $("#casLoginForm").find("#passwordEncrypt").val(_p2); } catch (e) { $("#casLoginForm").find("#passwordEncrypt").val(_p0); } }

    });
})(jQuery);

// 统一校验必填和展示错误信息的方法
function checkRequired(obj, msgId) {
    if (obj.length == 0) {
        return true;
    }

    if (obj.val() == "") {
        $("#" + msgId).show();
        return false;
    } else {
        $("#" + msgId).hide();
        return true;
    }
}

function GetQueryString(name) {
    var reg = new RegExp("(^|&)" + name + "=([^&]*)(&|$)");
    var r = window.location.search.substr(1).match(reg);
    if (r != null) return unescape(r[2]);
    return null;
}

function getCaptcha() {
    var username = $.trim($("#casLoginForm").find("#username").val());
    //    if ( username!= "" && $("#casLoginForm").find("#captchaResponse").length == 0) {
    if (username != "") {
        $.ajax("needCaptcha.html", {
            data: { username: username, pwdEncrypt2: "pwdEncryptSalt" },
            cache: false,
            dataType: "text",
            success: function (data) {
                if (data.indexOf("::::") > -1) {
                    var pwdEncryptArr = data.split("::::");
                    try { pwdDefaultEncryptSalt = pwdEncryptArr[1]; } catch (e) { }
                }
                if (data.indexOf("true") > -1) {
                    // 如果已经存在验证码，那么就不动
                    if ($("#casLoginForm").find("#captchaResponse").length != 0) {
                        return;
                    }
                    var casCaptcha = $("#cpatchaDiv");
                    casCaptcha.empty();
                    casCaptcha.html($("#hidenCaptchaDiv").html());
                    $("#casLoginForm").find("#changeCaptcha").bind("click", function () {
                        $("#casLoginForm").find("#captchaImg").attr("src", "captcha.html?ts=" + new Date().getMilliseconds());
                    });
                    $("#casLoginForm").find("#captchaImg").bind("click", function () {
                        $("#casLoginForm").find("#captchaImg").attr("src", "captcha.html?ts=" + new Date().getMilliseconds());
                    });
                    casCaptcha.fadeIn("slow");
                } else {
                    // 如果不需要验证码，并且验证码已经出现，那么清空
                    if ($("#casLoginForm").find("#captchaResponse").length != 0) {
                        var casCaptcha = $("#cpatchaDiv");
                        casCaptcha.empty();
                    }
                }
            }
        });
    }
}


/**
  * 倒计时函数
  *
  */
var buttonDefaultValue;//记录默认按钮值
function countDownButton(obj, second) {
    if (second == 120) {
        buttonDefaultValue = obj.value;
    }
    // 如果秒数还是大于0，则表示倒计时还没结束
    if (second >= 0) {
        // 按钮置为不可点击状态
        obj.disabled = true;
        // 按钮里的内容呈现倒计时状态
        obj.value = second + "s";
        // 时间减一
        second--;
        // 一秒后重复执行
        setTimeout(function () {
            countDownButton(obj, second);
        }, 1000);
        // 否则，按钮重置为初始状态
    } else {
        // 按钮置未可点击状态
        obj.disabled = false;
        // 按钮里的内容恢复初始状态
        obj.value = buttonDefaultValue;
    }
}

//发送验证码.
function sendDynamicCodeByPhone(username, authCodeTypeName, captchaVal) {

    $.ajax({
        type: "POST",
        url: "getDynamicCode.do",
        dataType: "json",
        data: { userName: username, authCodeTypeName: authCodeTypeName, captchaResponse: captchaVal },
        success: function (data) {
            var res = data.res;
            var returnMessage = data.returnMessage;
            var mobile = data.mobile;
            if (res == "success") {
                $("#sendCodeWarnMessage").text(returnMessage + mobile);
                countDownButton(document.getElementById("getDynamicCode"), 120);
            } else if (res == "wechat_success") {
                $("#sendCodeWarnMessage").text(returnMessage);
                countDownButton(document.getElementById("getWChatQYDynamicCode"), 120);
            } else if (res == "cpdaily_success") {
                $("#sendCodeWarnMessage").text(returnMessage);
                countDownButton(document.getElementById("getCpdailyDynamicCode"), 120);
            } else {
                $("#sendCodeWarnMessage").text(returnMessage);
                $("#casDynamicLoginForm").find("#username").focus();
                $("#casDynamicLoginForm").find("#dynamicCodeCaptchaImg").attr("src", "captcha.html?ts=" + new Date().getMilliseconds());
            }
        }
    });
}

//二维码登录
/*function ajaxGetQRCode() {
    $.ajax({
        type: "POST",
        url: "getQRUid.do",
        dataType: "json",
        success: function (data) {
            var uuid = data.qrUid;
            var qrCodeImage = $("#qrCodeImage");
            if (qrCodeImage != "" && qrCodeImage != null && qrCodeImage != 'undefined') {
                qrCodeImage.attr("src", "getUrlQRCode?uuid=" + uuid);
                $(".auth_tab_content_item[tabid=03]").find("#casLoginForm").find("#uuid").val(uuid);
                checkQRCodeStatus(uuid);

            }
        }
    });
}*/


/*function checkQRCodeStatus(uuid) {
    $("#appCodeRefresh").hide();
    $("#appCodeLoginLoad").hide();
    t = setInterval(function () {
        $.ajax({
            type: "GET",
            url: "qrCodeStatus.do",
            dataType: "json",
            data: {uuid: uuid},
            success: function (data) {
                var codeStatusType = data.codeStatus;
                var uid = data.uid;
                switch (codeStatusType) {
                    case 401:
                        break;
                    case 404:
                        $("#appCodeRefresh").show();
                        $("#appCodeRefresh").bind("click", function () {
                            ajaxGetQRCode();
                        });
                        clearInterval();
                        break;
                    case 200:
                        $("#appCodeLoginLoad").show();
                        $(".auth_tab_content_item[tabid=03]").find("#casLoginForm").submit();
                        break;
                    default :
                        alert("服务器维护中，请稍后再试");
                        break;
                }
            }
        });
    }, 5000);
}*/

function clearInterval() {
    clearTimeout(t);
}

//cs客户端集成-rtx

function rtxLogin() {
    try {
        var objKernalRoot = RTXAX.GetObject("KernalRoot");
        var objRtcData = objKernalRoot.Sign;
        var strAccount = objKernalRoot.Account;
        var strSgin = objRtcData.GetString("Sign");
        if (strAccount != "" && strAccount.length > 0 && strSgin != "" && strSgin.length > 0) {
            alert("欢迎使用rtx登陆：" + strAccount);
            csLogin(strAccount, strSgin);
        }
    } catch (e) {
        // alert(e);
    }
}

function csLogin(userId, csSignKey) {
    $.ajax({
        type: "POST",
        url: "rtxCombinedUserCheck.do",
        dataType: "json",
        data: { userId: userId },
        success: function (data) {
            var res = data.res;
            if (res == 1) {
                var rtxFrm = $(".auth_tab_content_item[tabid=04]").find("#rtxLoginForm");
                rtxFrm.find("#csUserId").val(userId);
                rtxFrm.find("#csSignKey").val(csSignKey);
                ///alert("欢迎使用rtx登陆：" + strAccount);
                rtxFrm.submit();

            } else {

            }
        }
    });
}