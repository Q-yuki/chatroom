const { invoke } = window.__TAURI__.core;

// register.js
document.getElementById('register-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const username = document.getElementById('register-username').value;
    const password = document.getElementById('register-password').value;

    // 基础输入验证
    if (!username || !password) {
        alert('用户名和密码不能为空');
        return;
    }

    try {
        const response = await invoke('register', {
            username: username,
            password: password
        });

        if (response.success) {
            // 注册成功后自动跳转登录页
            alert('注册成功，请登录');
            window.location.href = 'login.html';
        } else {
            // 显示错误提示
            alert(response.message);
            document.getElementById('register-password').value = '';
        }
    } catch (err) {
        console.error('注册请求错误:', err);
        alert(`注册失败: ${err.message || '服务器错误'}`);
    }
});

// 输入框回车键支持
document.getElementById('register-password').addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        document.getElementById('register-form').requestSubmit();
    }
});