const { invoke } = window.__TAURI__.core;

// login.js
document.getElementById('login-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const username = document.getElementById('login-username').value;
    const password = document.getElementById('login-password').value;

    try {
        const response = await invoke('login', { 
            username, 
            password 
        });
        
        if (response.success) {
            alert(response.message);
            window.location.href = `./room_management/room-management.html?username=${encodeURIComponent(username)}&room_id=${encodeURIComponent(response.roomId)}`;
        } else {
            alert(response.message);
        }
    } catch (err) {
        alert(`登录失败: ${err}`);
    }
});