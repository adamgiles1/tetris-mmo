let connected = false;
let ws;
let currentPlayerId;
let rightPressed = false;
let leftPressed = false;
let upPressed = false;
let downPressed = false;
let spacePressed = false;
let gameOver = false;
let winner = false;
let music;

function initializeBoards(playerIds) {
    let gameWindow = document.getElementById("game-window");
    let board = '<table id="game-board">';
    
    for (let h = 19; h >= 0; h--) {
        board += `<tr id="player-${this.currentPlayerId}-row-${h}">`;

        for (let w = 0; w <= 9; w++) {
            board += `<td id="player-${this.currentPlayerId}-tile-${h}-${w}"></td>`;
        }

        board += '</tr>';
    }



    board += '</table>';
    gameWindow.innerHTML = board;

    let leftGamesWindow = document.getElementById("left-game-window");
    let rightGamesWindow = document.getElementById("right-game-window");
    leftGamesWindow.innerHTML = "";
    let nextBoardIsLeft = true;

    for (playerId in playerIds) {
        // Don't generate small side board for main player
        if (playerId == this.currentPlayerId) {
            continue;
        }

        board = `<table class="other" id="game-board-${playerId}">`;
        for (let h = 19; h >= 0; h--) {
            board += `<tr id="player-${playerId}-row-${h}">`;

            for (let w = 0; w <= 9; w++) {
                board += `<td class="small" id="player-${playerId}-tile-${h}-${w}"></td>`;
            }

            board += '</tr>';
        }
        board += '</table>';
        
        if (nextBoardIsLeft) {
            leftGamesWindow.innerHTML += board;
        } else {
            rightGamesWindow.innerHTML += board;
        }
        nextBoardIsLeft = !nextBoardIsLeft;
    }
}

function updateAllBoards(boardsMessage) {

    let boards = boardsMessage.boards;
    for (let i = 0; i < boards.length; i++) {
        let board = boards[i];
        console.log(board);
        updateBoard(board);

        // Check if player has lost
        if (board.playerId === this.currentPlayerId) {
            if (board.gameEnded && !this.gameOver) {
                this.gameOver = true;
                loserSound();
                alert("loser");
            }
        }

        console.log("here it is: " + !this.winner + !this.gameOver + otherPlayersAllLost(boards));
        // Check if player has won
        if (!this.winner && !this.gameOver && otherPlayersAllLost(boards)) {
            winnerSound();
            this.winner = true;
        }
    }
}

function updateBoard(board) {
    console.log("board");
    console.log(board);
    // Update placed pieces
    for (let h = 19; h >= 0; h--) {
        for (let w = 9; w >= 0; w--) {
            setTile(h, w, board.tiles[w][h], board.playerId);
        }
    }

    // Update player controlled piece
    board.piece.positions.forEach(position => {
        setTile(position.y, position.x, board.piece.color, board.playerId);
    });
}

function setTile(h, w, pieceType, playerId) {
    let tile = document.getElementById(`player-${playerId}-tile-${h}-${w}`);
    let color;

    switch(pieceType) {
        case "I":
            color = "cyan";
            break;
        case "J":
            color = "blue";
            break;
        case "L":
            color = "orange";
            break;
        case "O":
            color = "yellow";
            break;
        case "S":
            color = "green";
            break;
        case "T": 
            color = "purple";
            break;
        case "Z":
            color = "red";
            break;
        case "B":
            color = "black";
            break;
        default:
            color = "grey";
    }

    if (tile) {
        tile.style.backgroundColor = color;
    }
}

function otherPlayersAllLost(boards) {
    for (board in boards) {
        if (!board.gameEnded && board.playerId != this.currentPlayerId) {
            return false;
        }
    }
    return true;
}

function startMusic() {
    this.music = new Audio("https://vgmsite.com/soundtracks/tetris-gameboy-rip/mpkrawiu/tetris-gameboy-02.mp3");
    this.music.loop = true;
    this.music.volume = .1;
    this.music.play();
}

function loserSound() {
    this.music.pause();
    this.music.stop;
    let sound = new Audio("https://vgmsite.com/soundtracks/bs-zelda-1996-snes/xfufsxrmcq/18%20Defeated.mp3");
    sound.play();
}

function winnerSound() {
    this.music.pause();
    this.music.stop;
    alert("winner");
    let sound = new Audio("https://vgmsite.com/soundtracks/bs-zelda-1996-snes/uuhmkimpun/12%20Triforce%20of%20Power.mp3");
    sound.play();
}

function connectToServer() {
    //make websocket connection to server
    //get sent back player id
    //start listening to player key inputs to send to server
    console.log("attempting to connect");
    openWebSocket();
    document.getElementById("connect-btn").disabled = true;
}

function enterMatchmaking() {
    console.log("entering matchmaking queue");
    sendCommand("QUEUE");
}

function startGame() {
    console.log("telling server to start game");
    sendCommand("START");
}

function sendCommand(command) {
    let message = JSON.stringify({
        msgType: "COMMAND",
        playerId: currentPlayerId,
        action: command
    });
    ws.send(message);
}

function startRecordingInputs() {
    document.addEventListener('keydown', keyDownHandler, false);
    document.addEventListener('keyup', keyUpHandler, false);
    console.log("recording inputs");
}

function keyDownHandler(key) {
    switch(key.keyCode) {
        case 39:
            rightPressed = true;
            sendInput("R");
            break;
        case 37:
            leftPressed = true;
            sendInput("L");
            break;
        case 38:
            upPressed = true;
            sendInput("U");
            break;
        case 40:
            downPressed = true;
            sendInput("D");
            break;
        case 32:
            spacePressed = true;
            sendInput("S");
            break;
        case 90:
            zPressed = true;
            sendInput("Z")
            break;
        case 88:
            xPressed = true;
            sendInput("X");
            break;
    }
}

function keyUpHandler(key) {
    switch(key.keycode) {
        case 39:
            rightPressed = false;
            break;
        case 37:
            leftPressed = false;
            break;
        case 38:
            upPressed = false;
            break;
        case 40:
            downPressed = false;
            break;
        case 32:
            spacePressed = false;
            break;
        case 90:
            zPressed = false;
            break;
        case 88:
            xPressed = false;
            break;
    }
}

function handleIncomingMessage(incomingMessage) {
    console.log("message received: " + incomingMessage);
    incomingMessage = JSON.parse(incomingMessage);
    switch(incomingMessage.msgType) {
        case "BOARD":
            updateAllBoards(incomingMessage);
            break;
        case "GAME_START":
            this.currentPlayerId = incomingMessage.playerId;
            console.log("Player id is: " + this.currentPlayerId);
            initializeBoards(incomingMessage.allIds);
            startMusic();
            this.gameOver = false;
            this.winner = false;
            break;
        default:
            console.log("unknown message type");
    }
}

function sendInput(input) {
    //if (!connected) return;
    
    let message = JSON.stringify({
        msgType: "INPUT",
        playerId: this.currentPlayerId,
        action: input
    });
    console.log(message);
    ws.send(message);
}

function openWebSocket() {
    let port = 6868;
    let ip = document.getElementById("ipInput").value;
    ws = new WebSocket("ws://" + ip + ":" + port);

    // Initialize web socket functions
    ws.onopen = function() {
        // web socket is connected
        console.log("connecting");
        this.connected = true;
    }

    ws.onmessage = function(incomingMessage) {
        handleIncomingMessage(incomingMessage.data);
    }

    ws.onclose =function() {
        alert("disconnected");
        this.connected = false;
        document.getElementById("connect-btn").disabled = false;
    }
}

startRecordingInputs();
