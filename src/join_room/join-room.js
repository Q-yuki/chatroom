const { invoke } = window.__TAURI__.core;

// 解析 URL 查询参数
function getQueryParam(name) {
    const params = new URLSearchParams(window.location.search);
    return params.get(name);
}

const username = getQueryParam('username');

document.addEventListener('DOMContentLoaded', () => {
    const currentUser = getQueryParam('username');
    const usernameSpan = document.querySelector('.user-info');
    if (usernameSpan && currentUser) {
        usernameSpan.textContent = currentUser;
    }
});

// join-room.js
document.getElementById('join-room-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const roomId = document.getElementById('room-id').value;

    try {
        const response = await invoke('join_room', {
            username,
            roomId
        });

        if (response.success) {
            window.location.href = `/chat/chat.html?username=${encodeURIComponent(username)}&room_id=${encodeURIComponent(roomId)}`;
        } else {
            alert(response.message);
        }
    } catch (err) {
        alert(`加入房间失败: ${err}`);
    }
});