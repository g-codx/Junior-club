import '../App.css';
import NavigationComponent from "../component/Navigation";
import PostComponent from "../component/Post";


function Post() {
    return (
        <div className="App">
            <NavigationComponent/>
            <PostComponent/>
        </div>
    );
}

export default Post;