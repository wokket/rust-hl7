using System;
using System.Runtime.InteropServices;

namespace ConsoleApp1
{
    public class Native
    {

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.LPWStr)]
        internal static extern string ping();

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "add")]
        internal static extern int Add(int a, int b);

        /// <summary>
        /// declared in rusthl7.dll
        /// </summary>
        [StructLayout(LayoutKind.Sequential)]
        public struct TestStruct
        {
            public int count;
        }

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "add_to_struct")]
        internal static extern int AddToStruct(IntPtr a, int b);

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "free_struct")]
        internal static extern void FreeStruct(IntPtr ptr);

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "new_test_struct_ref")]
        internal static extern IntPtr NewTestStructRef(); //points to a test struct

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "build_message")]
        internal static extern MessageHandle BuildMessage(string msg);

        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "get_field")]
        internal static extern StringHandle GetField(IntPtr ptrToMessage);
        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "free_string")]
        internal static extern void FreeString(IntPtr ptr);
        [DllImport("rusthl7.dll", CallingConvention = CallingConvention.Cdecl, EntryPoint = "free_message")]
        internal static extern void FreeMessage(IntPtr ptr);

    }
}
