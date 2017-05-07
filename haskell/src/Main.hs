import Data.List
data PizzaBase = Thin | Thick deriving (Enum, Show)
data Topping = Tomatoe | Sweetcorn | Peppers | Olives deriving (Enum, Show)
type Pizza = (PizzaBase, [Topping])

pizzaA :: Pizza
pizzaA = (Thin, [Tomatoe, Sweetcorn])

addTopping :: Pizza -> [Topping] -> Pizza
addTopping (base, toppings) toppingList = (base, toppings ++ toppingList)

pizzaB = addTopping pizzaA [Peppers, Olives]

strToppings :: [Topping] -> String
strToppings toppings = concat (intersperse ", " (map show toppings))
strPizza :: Pizza -> String
strPizza (base, toppings) = (show base) ++ strToppings toppings

pickABaseFromList :: Int -> [PizzaBase] -> PizzaBase
pickABaseFromList index bases = bases !! index

pickABase = (\index -> pickABaseFromList index [Thin ..])

main = do
	inputdata <- getContents
	putStrLn $ strPizza pizzaB
	putStrLn $ inputdata
