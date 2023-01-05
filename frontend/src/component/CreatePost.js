


function CreatPostComponent() {
    return (
    <div className="col">
        <div>
            <span className="edit material-icons">edit_document</span>
            <input className="input" placeholder="Create Post" type="text" onClick={() => window.location.href='/create'}/>
        </div>
    </div>
    );
}

export default CreatPostComponent;
