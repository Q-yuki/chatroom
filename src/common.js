// common.js (公共函数)
// 用户状态管理
let currentUser = localStorage.getItem('currentUser') || null;
let currentRoomId = localStorage.getItem('currentRoomId') || null;

// 初始化用户显示
document.addEventListener('DOMContentLoaded', () => {
    if(document.getElementById('username-display')) {
        document.getElementById('username-display').textContent = currentUser;
    }
    if(document.getElementById('current-room-id')) {
        document.getElementById('current-room-id').textContent = currentRoomId;
    }
});