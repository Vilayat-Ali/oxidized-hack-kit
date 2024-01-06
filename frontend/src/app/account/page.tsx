// lib
import Image from "next/image";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

// components
import { Card, CardContent, CardHeader } from "@/components/ui/card";

// form components
import Signup from "./forms/Signup";
import Login from "./forms/Login";

// ICONS
import { FaBurn } from "react-icons/fa";

const page = () => {
  return (
    <div className="flex flex-row justify-start items-center w-[100vw] h-[100vh] overflow-hidden">
      {/* Image */}
      <div className="relative hidden md:flex w-[50vw] h-[100%] shadow items-center justify-center bg-black text-white flex-col">
        {/* UNCOMMENT BELOW TO HAVE IMAGE  */}
        {/* <Image src="/next.svg" fill={true} alt="image" priority /> */}
        <FaBurn className="text-[10vw]" />
        <h1 className="my-5 text-2xl font-bold">Build Your App</h1>
      </div>
      {/* Image */}

      {/* Form */}
      <div className="w-[90vw] md:w-[50vw] h-[100%] flex flex-row justify-center items-center mx-auto">
        <Card className="w-[100%] md:w-[60%]">
          <Tabs defaultValue="Sign Up" className="w-[100%]">
            <CardHeader>
              <TabsList className="text-2xl">
                <TabsTrigger className="w-[50%]" value="Sign Up">
                  Sign Up
                </TabsTrigger>
                <TabsTrigger className="w-[50%]" value="Login">
                  Login
                </TabsTrigger>
              </TabsList>
            </CardHeader>

            <CardContent>
              {/* Signup */}
              <TabsContent value="Sign Up">
                <Signup />
              </TabsContent>
              {/* Signup */}

              {/* Login */}
              <TabsContent value="Login">
                <Login />
              </TabsContent>
              {/* Login */}
            </CardContent>
          </Tabs>
        </Card>
      </div>
      {/* Form */}
    </div>
  );
};

export default page;
