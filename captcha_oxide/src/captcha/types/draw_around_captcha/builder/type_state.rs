pub struct MissingBody;
pub struct MissingComment;
pub struct MissingImgInstructions;

pub struct BodyProvided<'a>(pub &'a str);
pub struct CommentProvided<'a>(pub &'a str);
pub struct ImgInstructionsProvided<'a>(pub &'a str);
