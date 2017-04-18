import Data.List
data PizzaBase = Thin | Thick deriving Show
data Topping = Tomatoe | Sweetcorn | Peppers | Olives deriving Show
type Pizza = (PizzaBase, [Topping])

pizzaA :: Pizza
pizzaA = (Thin, [Tomatoe, Sweetcorn])

addTopping :: Pizza -> Topping -> Pizza
addTopping (base, toppings) topping = (base, toppings ++ [topping])

pizzaB = addTopping pizzaA Peppers

strToppings :: [Topping] -> String
strToppings toppings = concat (intersperse ", " (map show toppings))
strPizza :: Pizza -> String
strPizza (base, toppings) = (show base) ++ strToppings toppings

main = do
	inputdata <- getContents
	putStrLn $ strPizza pizzaB
	putStrLn $ inputdata
