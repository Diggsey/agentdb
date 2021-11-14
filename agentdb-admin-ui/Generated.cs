using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace AgentdbAdmin
{    
    public static class AgentdbAdmin
    {
        public class RustException: Exception {
            public RustException(string message) : base(message) { }
        }

        public interface IOpaqueHandle: IEquatable<IOpaqueHandle>, IDisposable {}

        
        public struct ClientDesc {
            public DateTime lastActiveTs;
            public string name;
            public (uint,uint) partitions;
        }
        public struct MessageDesc {
            public Guid messageId;
            public Guid recipientId;
            public Nullable<DateTime> scheduledFor;
        }
        public struct PartitionDesc {
            public long agentCount;
            public List<MessageDesc> pendingMessages;
            public bool pendingMessagesOverflow;
            public List<MessageDesc> batchedMessages;
            public bool batchedMessagesOverflow;
        }
        public struct RootDesc {
            public (uint,uint) partitionRangeRecv;
            public (uint,uint) partitionRangeSend;
            public List<ClientDesc> clients;
            public SortedDictionary<uint,PartitionDesc> partitions;
        }
        public static IOpaqueHandle Connect(
            string path
        ) {
            return _DecodeResult(_FnConnect(_EncodeOption(path, _arg1 => _AllocStr(_arg1))), _arg2 => new _OpaqueHandle(_arg2));
        }
        public static void ListRoots(
            IOpaqueHandle con,
            Action<List<List<byte>>,string> continuation
        ) {
            _FnListRoots(((_OpaqueHandle)(con)).ToInner(5270083730820996706),((Func<Action<List<List<byte>>,string>, _RawDelegate>)(_arg3 => _AllocDelegate(new _LocalDelegate8((_arg3_arg0,_arg3_arg1) => _arg3(_DecodeOption(_arg3_arg0, _arg4 => _FreeSlice<List<byte>, _RawSlice, List<List<byte>>>(_arg4, 16, 8, _arg5 => _FreeSlice<byte, byte, List<byte>>(_arg5, 1, 1, _arg6 => _arg6))),_DecodeOption(_arg3_arg1, _arg7 => _FreeStr(_arg7)))), _arg3)))(continuation));
        }
        public static void DescribeRoot(
            IOpaqueHandle con,
            IReadOnlyCollection<byte> root,
            Action<RootDesc,string> continuation
        ) {
            _FnDescribeRoot(((_OpaqueHandle)(con)).ToInner(5270083730820996706),_AllocSlice<byte, byte>(root, 1, 1, _arg9 => _arg9),((Func<Action<RootDesc,string>, _RawDelegate>)(_arg10 => _AllocDelegate(new _LocalDelegate13((_arg10_arg0,_arg10_arg1) => _arg10(_DecodeOption(_arg10_arg0, _arg11 => (_arg11).Decode()),_DecodeOption(_arg10_arg1, _arg12 => _FreeStr(_arg12)))), _arg10)))(continuation));
        }
        public static void LoadBlob(
            IOpaqueHandle con,
            IReadOnlyCollection<byte> root,
            Guid blobId,
            Action<List<byte>,string> continuation
        ) {
            _FnLoadBlob(((_OpaqueHandle)(con)).ToInner(5270083730820996706),_AllocSlice<byte, byte>(root, 1, 1, _arg14 => _arg14),blobId,((Func<Action<List<byte>,string>, _RawDelegate>)(_arg15 => _AllocDelegate(new _LocalDelegate20((_arg15_arg0,_arg15_arg1) => _arg15(_DecodeOption(_arg15_arg0, _arg16 => _DecodeOption(_arg16, _arg17 => _FreeSlice<byte, byte, List<byte>>(_arg17, 1, 1, _arg18 => _arg18))),_DecodeOption(_arg15_arg1, _arg19 => _FreeStr(_arg19)))), _arg15)))(continuation));
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _StructClientDesc {
            public long lastActiveTs;
            public _RawSlice name;
            public _RawTuple3 partitions;
            public static _StructClientDesc Encode(ClientDesc structArg) {
                return new _StructClientDesc {
                    lastActiveTs = (structArg.lastActiveTs).ToUniversalTime().Ticks,
                    name = _AllocStr(structArg.name),
                    partitions = ((Func<(uint,uint), _RawTuple3>)(_arg21 => new _RawTuple3 { elem0 = _arg21.Item1,elem1 = _arg21.Item2 }))(structArg.partitions)
                };
            }
            public ClientDesc Decode() {
                return new ClientDesc {
                    lastActiveTs = new DateTime(this.lastActiveTs, DateTimeKind.Utc),
                    name = _FreeStr(this.name),
                    partitions = ((Func<_RawTuple3, (uint,uint)>)(_arg22 => (_arg22.elem0,_arg22.elem1)))(this.partitions)
                };
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _StructMessageDesc {
            public Guid messageId;
            public Guid recipientId;
            public _RawTuple4 scheduledFor;
            public static _StructMessageDesc Encode(MessageDesc structArg) {
                return new _StructMessageDesc {
                    messageId = structArg.messageId,
                    recipientId = structArg.recipientId,
                    scheduledFor = _EncodeOption(structArg.scheduledFor, _arg23 => (_arg23.Value).ToUniversalTime().Ticks)
                };
            }
            public MessageDesc Decode() {
                return new MessageDesc {
                    messageId = this.messageId,
                    recipientId = this.recipientId,
                    scheduledFor = _DecodeOption(this.scheduledFor, _arg24 => new Nullable<DateTime>(new DateTime(_arg24, DateTimeKind.Utc)))
                };
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _StructPartitionDesc {
            public long agentCount;
            public _RawSlice pendingMessages;
            public byte pendingMessagesOverflow;
            public _RawSlice batchedMessages;
            public byte batchedMessagesOverflow;
            public static _StructPartitionDesc Encode(PartitionDesc structArg) {
                return new _StructPartitionDesc {
                    agentCount = structArg.agentCount,
                    pendingMessages = _AllocSlice<MessageDesc, _StructMessageDesc>(structArg.pendingMessages, 48, 8, _arg25 => _StructMessageDesc.Encode(_arg25)),
                    pendingMessagesOverflow = (structArg.pendingMessagesOverflow ? (byte)1 : (byte)0),
                    batchedMessages = _AllocSlice<MessageDesc, _StructMessageDesc>(structArg.batchedMessages, 48, 8, _arg26 => _StructMessageDesc.Encode(_arg26)),
                    batchedMessagesOverflow = (structArg.batchedMessagesOverflow ? (byte)1 : (byte)0)
                };
            }
            public PartitionDesc Decode() {
                return new PartitionDesc {
                    agentCount = this.agentCount,
                    pendingMessages = _FreeSlice<MessageDesc, _StructMessageDesc, List<MessageDesc>>(this.pendingMessages, 48, 8, _arg27 => (_arg27).Decode()),
                    pendingMessagesOverflow = (this.pendingMessagesOverflow != 0),
                    batchedMessages = _FreeSlice<MessageDesc, _StructMessageDesc, List<MessageDesc>>(this.batchedMessages, 48, 8, _arg28 => (_arg28).Decode()),
                    batchedMessagesOverflow = (this.batchedMessagesOverflow != 0)
                };
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _StructRootDesc {
            public _RawTuple3 partitionRangeRecv;
            public _RawTuple3 partitionRangeSend;
            public _RawSlice clients;
            public _RawSlice partitions;
            public static _StructRootDesc Encode(RootDesc structArg) {
                return new _StructRootDesc {
                    partitionRangeRecv = ((Func<(uint,uint), _RawTuple3>)(_arg29 => new _RawTuple3 { elem0 = _arg29.Item1,elem1 = _arg29.Item2 }))(structArg.partitionRangeRecv),
                    partitionRangeSend = ((Func<(uint,uint), _RawTuple3>)(_arg30 => new _RawTuple3 { elem0 = _arg30.Item1,elem1 = _arg30.Item2 }))(structArg.partitionRangeSend),
                    clients = _AllocSlice<ClientDesc, _StructClientDesc>(structArg.clients, 32, 8, _arg31 => _StructClientDesc.Encode(_arg31)),
                    partitions = _AllocDict<uint, PartitionDesc, _RawTuple5>(structArg.partitions, 64, 8, _arg32 => ((Func<(uint,PartitionDesc), _RawTuple5>)(_arg33 => new _RawTuple5 { elem0 = _arg33.Item1,elem1 = _StructPartitionDesc.Encode(_arg33.Item2) }))(_arg32))
                };
            }
            public RootDesc Decode() {
                return new RootDesc {
                    partitionRangeRecv = ((Func<_RawTuple3, (uint,uint)>)(_arg34 => (_arg34.elem0,_arg34.elem1)))(this.partitionRangeRecv),
                    partitionRangeSend = ((Func<_RawTuple3, (uint,uint)>)(_arg35 => (_arg35.elem0,_arg35.elem1)))(this.partitionRangeSend),
                    clients = _FreeSlice<ClientDesc, _StructClientDesc, List<ClientDesc>>(this.clients, 32, 8, _arg36 => (_arg36).Decode()),
                    partitions = _FreeDict<uint, PartitionDesc, _RawTuple5, SortedDictionary<uint, PartitionDesc>>(this.partitions, 64, 8, _arg37 => ((Func<_RawTuple5, (uint,PartitionDesc)>)(_arg38 => (_arg38.elem0,(_arg38.elem1).Decode())))(_arg37))
                };
            }
        }
        [DllImport("agentdb_admin", EntryPoint = "rnet_export_connect", CallingConvention = CallingConvention.Cdecl)]
        private static extern _RawTuple6 _FnConnect(
            _RawTuple0 path
        );
        [DllImport("agentdb_admin", EntryPoint = "rnet_export_list_roots", CallingConvention = CallingConvention.Cdecl)]
        private static extern void _FnListRoots(
            _RawOpaqueHandle con,
            _RawDelegate continuation
        );
        [DllImport("agentdb_admin", EntryPoint = "rnet_export_describe_root", CallingConvention = CallingConvention.Cdecl)]
        private static extern void _FnDescribeRoot(
            _RawOpaqueHandle con,
            _RawSlice root,
            _RawDelegate continuation
        );
        [DllImport("agentdb_admin", EntryPoint = "rnet_export_load_blob", CallingConvention = CallingConvention.Cdecl)]
        private static extern void _FnLoadBlob(
            _RawOpaqueHandle con,
            _RawSlice root,
            Guid blobId,
            _RawDelegate continuation
        );
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] delegate void _LocalDelegate8(_RawTuple0 arg0,_RawTuple0 arg1);
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] delegate void _LocalDelegate13(_RawTuple1 arg0,_RawTuple0 arg1);
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] delegate void _LocalDelegate20(_RawTuple2 arg0,_RawTuple0 arg1);
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple0 {
            public _RawSlice elem0;
            public byte elem1;
        }
        private static _RawTuple0 _EncodeOption<T>(T arg, Func<T, _RawSlice> converter) {
            if (arg != null) {
                return new _RawTuple0 { elem0 = converter(arg), elem1 = 1 };
            } else {
                return new _RawTuple0 { elem0 = default(_RawSlice), elem1 = 0 };
            }
        }
        private static T _DecodeOption<T>(_RawTuple0 arg, Func<_RawSlice, T> converter) {
            if (arg.elem1 != 0) {
                return converter(arg.elem0);
            } else {
                return default(T);
            }
        }
        private static _RawTuple0 _EncodeResult(Action f) {
            try {
                f();
                return new _RawTuple0 { elem0 = default(_RawSlice), elem1 = 1 };
            } catch (Exception e) {
                return new _RawTuple0 { elem0 = _AllocStr(e.Message), elem1 = 0 };
            }
        }
        private static void _DecodeResult(_RawTuple0 arg) {
            if (arg.elem1 == 0) {
                throw new RustException(_FreeStr(arg.elem0));
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple1 {
            public _StructRootDesc elem0;
            public byte elem1;
        }
        private static _RawTuple1 _EncodeOption<T>(T arg, Func<T, _StructRootDesc> converter) {
            if (arg != null) {
                return new _RawTuple1 { elem0 = converter(arg), elem1 = 1 };
            } else {
                return new _RawTuple1 { elem0 = default(_StructRootDesc), elem1 = 0 };
            }
        }
        private static T _DecodeOption<T>(_RawTuple1 arg, Func<_StructRootDesc, T> converter) {
            if (arg.elem1 != 0) {
                return converter(arg.elem0);
            } else {
                return default(T);
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple2 {
            public _RawTuple0 elem0;
            public byte elem1;
        }
        private static _RawTuple2 _EncodeOption<T>(T arg, Func<T, _RawTuple0> converter) {
            if (arg != null) {
                return new _RawTuple2 { elem0 = converter(arg), elem1 = 1 };
            } else {
                return new _RawTuple2 { elem0 = default(_RawTuple0), elem1 = 0 };
            }
        }
        private static T _DecodeOption<T>(_RawTuple2 arg, Func<_RawTuple0, T> converter) {
            if (arg.elem1 != 0) {
                return converter(arg.elem0);
            } else {
                return default(T);
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple3 {
            public uint elem0;
            public uint elem1;
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple4 {
            public long elem0;
            public byte elem1;
        }
        private static _RawTuple4 _EncodeOption<T>(T arg, Func<T, long> converter) {
            if (arg != null) {
                return new _RawTuple4 { elem0 = converter(arg), elem1 = 1 };
            } else {
                return new _RawTuple4 { elem0 = default(long), elem1 = 0 };
            }
        }
        private static T _DecodeOption<T>(_RawTuple4 arg, Func<long, T> converter) {
            if (arg.elem1 != 0) {
                return converter(arg.elem0);
            } else {
                return default(T);
            }
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple5 {
            public uint elem0;
            public _StructPartitionDesc elem1;
        }
        [StructLayout(LayoutKind.Sequential)]
        private struct _RawTuple6 {
            public _RawOpaqueHandle elem0;
            public _RawSlice elem1;
            public byte elem2;
        }
        private static _RawTuple6 _EncodeResult(Func<_RawOpaqueHandle> f) {
            try {
                var res = f();
                return new _RawTuple6 { elem0 = res, elem1 = default(_RawSlice), elem2 = 1 };
            } catch (Exception e) {
                return new _RawTuple6 { elem0 = default(_RawOpaqueHandle), elem1 = _AllocStr(e.Message), elem2 = 0 };
            }
        }
        private static T _DecodeResult<T>(_RawTuple6 arg, Func<_RawOpaqueHandle, T> converter) {
            if (arg.elem2 != 0) {
                return converter(arg.elem0);
            } else {
                throw new RustException(_FreeStr(arg.elem1));
            }
        }


        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate void _ManageDelegateDelegate(IntPtr ptr, int adjust);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate void _DropOpaqueDelegate(IntPtr ptr);

        private static Dictionary<IntPtr, (int, Delegate, Delegate)> _ActiveDelegates = new Dictionary<IntPtr, (int, Delegate, Delegate)>();

        private static _ManageDelegateDelegate _manageDelegate = _ManageDelegate;
        private static IntPtr _manageDelegatePtr = Marshal.GetFunctionPointerForDelegate(_manageDelegate);

        private static void _ManageDelegate(IntPtr ptr, int adjust)
        {
            lock (_ActiveDelegates)
            {
                var item = _ActiveDelegates[ptr];
                item.Item1 += adjust;
                if (item.Item1 > 0)
                {
                    _ActiveDelegates[ptr] = item;
                }
                else
                {
                    _ActiveDelegates.Remove(ptr);
                }
            }
        }

        private static _RawDelegate _AllocDelegate(Delegate d, Delegate original)
        {
            var ptr = Marshal.GetFunctionPointerForDelegate(d);
            lock (_ActiveDelegates)
            {
                if (_ActiveDelegates.ContainsKey(ptr))
                {
                    var item = _ActiveDelegates[ptr];
                    item.Item1 += 1;
                    _ActiveDelegates[ptr] = item;
                } else
                {
                    _ActiveDelegates.Add(ptr, (1, d, original));
                }
            }
            return new _RawDelegate
            {
                call_fn = ptr,
                drop_fn = _manageDelegatePtr,
            };
        }

        private static Delegate _FreeDelegate(_RawDelegate d)
        {
            var ptr = d.call_fn;
            lock (_ActiveDelegates)
            {
                var item = _ActiveDelegates[ptr];
                item.Item1 -= 1;
                if (item.Item1 > 0)
                {
                    _ActiveDelegates[ptr] = item;
                }
                else
                {
                    _ActiveDelegates.Remove(ptr);
                }
                return item.Item3;
            }
        }

        [DllImport("agentdb_admin", EntryPoint = "rnet_alloc", CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr _Alloc( UIntPtr size, UIntPtr align);

        [DllImport("agentdb_admin", EntryPoint = "rnet_free", CallingConvention = CallingConvention.Cdecl)]
        private static extern void _Free(IntPtr ptr, UIntPtr size, UIntPtr align);

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawSlice
        {
            public IntPtr ptr;
            public UIntPtr len;

            public static _RawSlice Alloc(UIntPtr len, int size, int align)
            {
                if (len == UIntPtr.Zero)
                {
                    return new _RawSlice {
                        ptr = (IntPtr)align,
                        len = UIntPtr.Zero,
                    };
                } else
                {
                    return new _RawSlice
                    {
                        ptr = _Alloc((UIntPtr)((UInt64)len * (UInt64)size), (UIntPtr)align),
                        len = len,
                    };
                }
            }

            public void Free(int size, int align)
            {
                if (len != UIntPtr.Zero)
                {
                    _Free(ptr, (UIntPtr)((UInt64)len * (UInt64)size), (UIntPtr)align);
                    ptr = (IntPtr)1;
                    len = UIntPtr.Zero;
                }
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawOpaqueHandle
        {
            public IntPtr ptr;
            public IntPtr drop_fn;
            public ulong type_id;

            public void Drop()
            {
                if (ptr != IntPtr.Zero)
                {
                    var drop = Marshal.GetDelegateForFunctionPointer<_DropOpaqueDelegate>(drop_fn);
                    drop(ptr);
                    ptr = IntPtr.Zero;
                }
            }
        }

        private class _OpaqueHandle : IOpaqueHandle
        {
            private _RawOpaqueHandle inner;

            public _OpaqueHandle(_RawOpaqueHandle inner)
            {
                this.inner = inner;
            }

            public _RawOpaqueHandle ToInner(ulong type_id)
            {
                if (type_id != inner.type_id)
                {
                    throw new InvalidCastException("Opaque handle does not have the correct type");
                }
                return this.inner;
            }

            ~_OpaqueHandle()
            {
                inner.Drop();
            }

            public override bool Equals(object obj)
            {
                return Equals(obj as _OpaqueHandle);
            }

            public bool Equals(IOpaqueHandle other)
            {
                var casted = other as _OpaqueHandle;
                return casted != null &&
                       inner.ptr == casted.inner.ptr && inner.type_id == casted.inner.type_id;
            }

            public override int GetHashCode()
            {
                return inner.ptr.GetHashCode() + inner.type_id.GetHashCode();
            }

            public void Dispose()
            {
                inner.Drop();
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct _RawDelegate
        {
            public IntPtr call_fn;
            public IntPtr drop_fn;
        }

        private static IntPtr _AllocBox<T>(T arg, int size, int align)
        {
            if (size > 0) {
                var ptr = _Alloc((UIntPtr)size, (UIntPtr)align);
                Marshal.StructureToPtr(arg, ptr, false);
                return ptr;
            } else {
                return (IntPtr)align;
            }
        }

        private static _RawSlice _AllocStr(string arg)
        {
            var nb = Encoding.UTF8.GetByteCount(arg);
            var slice = _RawSlice.Alloc((UIntPtr)nb, 1, 1);
            unsafe
            {
                fixed (char* firstChar = arg)
                {
                    nb = Encoding.UTF8.GetBytes(firstChar, arg.Length, (byte*)slice.ptr, nb);
                }
            }
            return slice;
        }

        private static _RawSlice _AllocSlice<T, U>(IReadOnlyCollection<T> collection, int size, int align, Func<T, U> converter) {
            var count = collection.Count;
            var slice = _RawSlice.Alloc((UIntPtr)count, size, align);
            var ptr = slice.ptr;
            foreach (var item in collection) {
                Marshal.StructureToPtr(converter(item), ptr, false);
                ptr = (IntPtr)(ptr.ToInt64() + (long)size);
            }
            return slice;
        }

        private static _RawSlice _AllocDict<TKey, TValue, U>(IReadOnlyDictionary<TKey, TValue> collection, int size, int align, Func<(TKey, TValue), U> converter) where U: unmanaged
        {
            var count = collection.Count;
            var slice = _RawSlice.Alloc((UIntPtr)count, size, align);
            var ptr = slice.ptr;
            foreach (var item in collection)
            {
                Marshal.StructureToPtr<U>(converter((item.Key, item.Value)), ptr, false);
                ptr = (IntPtr)(ptr.ToInt64() + (long)size);
            }
            return slice;
        }

        private static T _FreeBox<T>(IntPtr ptr, int size, int align)
        {
            var res = Marshal.PtrToStructure<T>(ptr);
            if (size > 0) {
                _Free(ptr, (UIntPtr)size, (UIntPtr)align);
            }
            return res;
        }

        private static String _FreeStr(_RawSlice arg)
        {
            unsafe
            {
                var res = Encoding.UTF8.GetString((byte*)arg.ptr, (int)arg.len);
                arg.Free(1, 1);
                return res;
            }
        }

        private static TList _FreeSlice<T, U, TList>(_RawSlice arg, int size, int align, Func<U, T> converter) where TList: ICollection<T>, new()
        {
            unsafe
            {
                var res = new TList();
                var ptr = arg.ptr;
                for (var i = 0; i < (int)arg.len; ++i) {
                    res.Add(converter(Marshal.PtrToStructure<U>(ptr)));
                    ptr = (IntPtr)(ptr.ToInt64() + (long)size);
                }
                arg.Free(size, align);
                return res;
            }
        }

        private static TDict _FreeDict<TKey, TValue, U, TDict>(_RawSlice arg, int size, int align, Func<U, (TKey, TValue)> converter) where U : unmanaged where TDict: IDictionary<TKey, TValue>, new()
        {
            unsafe
            {
                var res = new TDict();
                var ptr = arg.ptr;
                for (var i = 0; i < (int)arg.len; ++i)
                {
                    var item = converter(Marshal.PtrToStructure<U>(ptr));
                    res.Add(item.Item1, item.Item2);
                    ptr = (IntPtr)(ptr.ToInt64() + (long)size);
                }
                arg.Free(size, align);
                return res;
            }
        }
    }
}
