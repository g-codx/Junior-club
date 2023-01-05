import React from 'react';
import ReactDOM from 'react-dom/client';
import 'bootstrap/dist/css/bootstrap.min.css';
import {BrowserRouter, Route, Routes} from 'react-router-dom';

import './index.css';
import Main from './page/Main.js';
import Java from "./page/Java";
import Rust from "./page/Rust";
import LogIn from "./page/LogIn";
import Post from "./page/Post";
import CreatePost from "./page/CreatePost";
import Registration from "./page/Registration";

const root = ReactDOM.createRoot(document.getElementById('root'));


root.render(
    <BrowserRouter>
        <Routes>
            <Route path="/" element={<Main />}/>
            <Route path="/java" element={<Java />}/>
            <Route path="/rust" element={<Rust />}/>
            <Route path="/login" element={<LogIn />}/>
            <Route path="/post/*" element={<Post />}/>
            <Route path="/create" element={<CreatePost />}/>
            <Route path="/registration" element={<Registration />}/>
        </Routes>
    </BrowserRouter>
);
