import Text.ParserCombinators.Parsec

parseSingleLineComment :: Parser String
parseSingleLineComment = do 
    try $ string "//" 
    many $ noneOf "\n"

parseMultilineComment :: Parser String
parseMultilineComment = do
    try $ string "/*" 
    manyTill anyChar (try $ string "*/")
    
parseEndOfFile :: Parser String
parseEndOfFile = do 
  x <- eof
  return ""

parseComment :: Parser String
parseComment = parseSingleLineComment <|> parseMultilineComment
    
parseNotComment :: Parser String
parseNotComment = manyTill anyChar (lookAhead (parseComment <|> parseEndOfFile))

extractComments :: Parser [String]
extractComments = do
  parseNotComment
  xs <- sepEndBy parseComment parseNotComment
  eof
  return xs


printHelperF :: String -> IO ()
printHelperF s = do
  print s
  print $ parse extractComments "Test Parser" s
  print "-------------------"

-- main
main :: IO ()
main = do 
  let samples = [
                  "No comments here",
                  "//Hello there!\n//General Kenobi",
                  "/* What's the deal with airline food?\nIt keeps getting worse and worse\nI can't take it anymore!*/",
                  " //Global Variable\nlet x = 5;\n/*TODO:\n\t// Add the number of cats as a variable\n\t//Shouldn't take too long\n*/\nlet c = 500;",
                  "//First\n//Second//NotThird\n//Third",
                  "x = 3*4 /* not 3*5 */",
                  "/* foo */ /* unterminated comment",
                  ""
                ]
  mapM_ printHelperF samples


-- > runhaskell test.hs
-- "No comments here"
-- Left "Test Parser" (line 1, column 17):
-- unexpected end of input
-- expecting "//" or "/*" <---------- fails because no comment in string
-- "-------------------"
-- "//Hello there!\n//General Kenobi"
-- Right ["Hello there!"] <---------- fails to extract the last comment
-- "-------------------"
-- "/* What's the deal with airline food?\nIt keeps getting worse and worse\nI can't take it anymore!*/"
-- Right [" What's the deal with airline food?\nIt keeps getting worse and worse\nI can't take it anymore!"] <- correct
-- "-------------------"
-- " //Global Variable\nlet x = 5;\n/*TODO:\n\t// Add the number of cats as a variable\n\t//Shouldn't take too long\n*/\nlet c = 500;"
-- Right ["Global Variable","TODO:\n\t// Add the number of cats as a variable\n\t//Shouldn't take too long\n"] <- correct
-- "-------------------"
-- "//First\n//Second//NotThird\n//Third"
-- Right ["First","Second//NotThird"] <- again fails to extract the last comment
-- "-------------------"