const { invoke } = window.__TAURI__.core;

// 解析 URL 查询参数
function getQueryParam(name) {
    const params = new URLSearchParams(window.location.search);
    return params.get(name);
}

const username = getQueryParam('username');
const roomId = getQueryParam('room_id');

document.addEventListener('DOMContentLoaded', () => {
    const roomId = getQueryParam('room_id');
    document.getElementById('current-room-id').textContent = roomId;
});

const ws = new WebSocket(`ws://127.0.0.1:8888/?room_id=${roomId}&username=${username}`);

ws.onerror = (error) => {
    console.error('WebSocket Error: ', error);
    alert('Failed to connect to the chat room. Please check the server configuration.');
};

ws.onopen = () => {
    console.log('WebSocket connection established.');
};

setInterval(() => {
    invoke('show_member_num', { roomId })
        .then((result) => {
            document.getElementById('online-count').textContent = result;
        })
        .catch((err) => {
            console.error('定时获取成员数量失败:', err);
        });
}, 50);

//消息接收与渲染
ws.onmessage = (event) => {
    try {
        const messageData = JSON.parse(event.data);
        if (roomId !== messageData.room_id) {
            console.warn('房间号不匹配，忽略消息');
            return;
        }

        const isSelf = messageData.username === username;
        const messageBubble = document.createElement('div');
        messageBubble.className = `message-bubble ${isSelf ? 'self' : 'other'}`;

        const messageContent = document.createElement('div');
        messageContent.className = 'message-content';

        // 始终显示用户名
        const senderName = document.createElement('span');
        senderName.className = 'sender-name';
        senderName.textContent = messageData.username;
        messageContent.appendChild(senderName);

        const bubbleContent = document.createElement('div');
        bubbleContent.className = 'bubble-content';

        if (messageData.type === "image") {
            // 图片内容
            const img = document.createElement('img');
            img.src = messageData.image_data;
            img.alt = "图片消息";
            img.style.maxWidth = "200px";
            img.style.display = "block";
            bubbleContent.appendChild(img);
        } else {
            // 文本内容
            bubbleContent.textContent = messageData.message;
        }
        messageContent.appendChild(bubbleContent);

        const timestamp = document.createElement('span');
        timestamp.className = 'timestamp';
        timestamp.textContent = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
        messageContent.appendChild(timestamp);

        messageBubble.appendChild(messageContent);

        document.getElementById('chat-messages').appendChild(messageBubble);
        document.getElementById('chat-messages').scrollTop = document.getElementById('chat-messages').scrollHeight;
    } catch (e) {
        console.warn('收到非JSON消息', event.data);
    }
};

document.getElementById('image-input').addEventListener('change', function (event) {
    const file = event.target.files[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = function (e) {
        // 发送图片消息
        ws.send(JSON.stringify({
            type: "image",
            room_id: roomId,
            username: username,
            image_data: e.target.result
        }));
    };
    reader.readAsDataURL(file);
});

function sendMessage() {
    const messageInput = document.getElementById('message-input');
    const message = messageInput.value.trim();
    if (message && ws.readyState === WebSocket.OPEN) {
        // 发送 JSON 格式，包含用户名和房间号
        ws.send(JSON.stringify({
            type: "chat",
            room_id: roomId,
            username: username,
            message: message
        }));
        messageInput.value = '';
    }
}

function leaveRoom() {
    invoke('leave_room', {
        username: username,
        roomId: roomId
    }).then(() => {
        // 跳转回房间管理页面，并带上用户名参数
        window.location.href = `/room_management/room-management.html?username=${encodeURIComponent(username)}`;
    }).catch((err) => {
        console.error('离开房间失败:', err);
        alert('离开房间失败，请稍后再试。');
    });
    ws.close();
}
