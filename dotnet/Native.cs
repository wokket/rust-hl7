using System;
using System.Runtime.InteropServices;

namespace ConsoleApp1
{
    public class Native
    {
        /// <summary>
        /// Builds a message in the Rust library, and returns a handle (pointer) that can be used in future API calls.
        /// Ensure this handle is disposed in a timely fashion.
        /// </summary>
        /// <param name="msg">A HL7 pipe-formatted string</param>
        /// <returns></returns>
        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "build_message")]
        internal static extern MessageHandle BuildMessage(string msg);

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "get_field")]
        internal static extern StringHandle GetField(IntPtr ptrToMessage, string segmentName, int fieldIndex);




    }
}
