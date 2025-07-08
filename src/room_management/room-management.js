const { invoke } = window.__TAURI__.core;

// 解析 URL 查询参数
function getQueryParam(name) {
    const params = new URLSearchParams(window.location.search);
    return params.get(name);
}

const username = getQueryParam('username');
document.addEventListener('DOMContentLoaded', () => {
    const currentUser = getQueryParam('username');
    const usernameSpan = document.querySelector('.username');
    if (usernameSpan && currentUser) {
        usernameSpan.textContent = currentUser;
    }
});

async function handleJoinRoom() {
    window.location.href = `/join_room/join-room.html?username=${encodeURIComponent(username)}`;
}

// room-management.js
async function handleCreateRoom() {
    const username = getQueryParam('username');
    try {
        const response = await invoke('create_room', {
            username: username
        });

        if (response.success) {
            // 跳转时带上 username 和 room_id
            roomId = response.room_id;
            alert(`房间创建成功，房间号: ${roomId}`);
            window.location.href = `/chat/chat.html?username=${encodeURIComponent(username)}&room_id=${encodeURIComponent(roomId)}`;
        }
    } catch (err) {
        alert(`创建房间失败: ${err}`);
    }
}

async function logout() {
    const username = getQueryParam('username');
    try {
        const response = await invoke('logout', {
            username: username
        });

        if (response.success) {
            alert(response.message);
            window.location.href = '/index.html';
        }
    } catch (err) {
        alert(`登出失败：${err}`);
    }
}