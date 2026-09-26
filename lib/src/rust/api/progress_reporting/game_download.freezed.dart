// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'game_download.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$DownloadGameProgress {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'DownloadGameProgress()';
}


}

/// @nodoc
class $DownloadGameProgressCopyWith<$Res>  {
$DownloadGameProgressCopyWith(DownloadGameProgress _, $Res Function(DownloadGameProgress) __);
}


/// Adds pattern-matching-related methods to [DownloadGameProgress].
extension DownloadGameProgressPatterns on DownloadGameProgress {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( DownloadGameProgress_Started value)?  started,TResult Function( DownloadGameProgress_FileSizeVerification value)?  fileSizeVerification,TResult Function( DownloadGameProgress_FileAllocationStarted value)?  fileAllocationStarted,TResult Function( DownloadGameProgress_FileAllocation value)?  fileAllocation,TResult Function( DownloadGameProgress_AllocationError value)?  allocationError,TResult Function( DownloadGameProgress_DownloadProgress value)?  downloadProgress,TResult Function( DownloadGameProgress_Finished value)?  finished,TResult Function( DownloadGameProgress_Cancelled value)?  cancelled,required TResult orElse(),}){
final _that = this;
switch (_that) {
case DownloadGameProgress_Started() when started != null:
return started(_that);case DownloadGameProgress_FileSizeVerification() when fileSizeVerification != null:
return fileSizeVerification(_that);case DownloadGameProgress_FileAllocationStarted() when fileAllocationStarted != null:
return fileAllocationStarted(_that);case DownloadGameProgress_FileAllocation() when fileAllocation != null:
return fileAllocation(_that);case DownloadGameProgress_AllocationError() when allocationError != null:
return allocationError(_that);case DownloadGameProgress_DownloadProgress() when downloadProgress != null:
return downloadProgress(_that);case DownloadGameProgress_Finished() when finished != null:
return finished(_that);case DownloadGameProgress_Cancelled() when cancelled != null:
return cancelled(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( DownloadGameProgress_Started value)  started,required TResult Function( DownloadGameProgress_FileSizeVerification value)  fileSizeVerification,required TResult Function( DownloadGameProgress_FileAllocationStarted value)  fileAllocationStarted,required TResult Function( DownloadGameProgress_FileAllocation value)  fileAllocation,required TResult Function( DownloadGameProgress_AllocationError value)  allocationError,required TResult Function( DownloadGameProgress_DownloadProgress value)  downloadProgress,required TResult Function( DownloadGameProgress_Finished value)  finished,required TResult Function( DownloadGameProgress_Cancelled value)  cancelled,}){
final _that = this;
switch (_that) {
case DownloadGameProgress_Started():
return started(_that);case DownloadGameProgress_FileSizeVerification():
return fileSizeVerification(_that);case DownloadGameProgress_FileAllocationStarted():
return fileAllocationStarted(_that);case DownloadGameProgress_FileAllocation():
return fileAllocation(_that);case DownloadGameProgress_AllocationError():
return allocationError(_that);case DownloadGameProgress_DownloadProgress():
return downloadProgress(_that);case DownloadGameProgress_Finished():
return finished(_that);case DownloadGameProgress_Cancelled():
return cancelled(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( DownloadGameProgress_Started value)?  started,TResult? Function( DownloadGameProgress_FileSizeVerification value)?  fileSizeVerification,TResult? Function( DownloadGameProgress_FileAllocationStarted value)?  fileAllocationStarted,TResult? Function( DownloadGameProgress_FileAllocation value)?  fileAllocation,TResult? Function( DownloadGameProgress_AllocationError value)?  allocationError,TResult? Function( DownloadGameProgress_DownloadProgress value)?  downloadProgress,TResult? Function( DownloadGameProgress_Finished value)?  finished,TResult? Function( DownloadGameProgress_Cancelled value)?  cancelled,}){
final _that = this;
switch (_that) {
case DownloadGameProgress_Started() when started != null:
return started(_that);case DownloadGameProgress_FileSizeVerification() when fileSizeVerification != null:
return fileSizeVerification(_that);case DownloadGameProgress_FileAllocationStarted() when fileAllocationStarted != null:
return fileAllocationStarted(_that);case DownloadGameProgress_FileAllocation() when fileAllocation != null:
return fileAllocation(_that);case DownloadGameProgress_AllocationError() when allocationError != null:
return allocationError(_that);case DownloadGameProgress_DownloadProgress() when downloadProgress != null:
return downloadProgress(_that);case DownloadGameProgress_Finished() when finished != null:
return finished(_that);case DownloadGameProgress_Cancelled() when cancelled != null:
return cancelled(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt totalFiles,  BigInt totalBytes)?  started,TResult Function( BigInt checkedFiles)?  fileSizeVerification,TResult Function( BigInt totalFiles,  BigInt totalBytes)?  fileAllocationStarted,TResult Function( BigInt allocatedFiles,  BigInt allocatedBytes)?  fileAllocation,TResult Function( String field0)?  allocationError,TResult Function( BigInt downloadedBytes)?  downloadProgress,TResult Function()?  finished,TResult Function()?  cancelled,required TResult orElse(),}) {final _that = this;
switch (_that) {
case DownloadGameProgress_Started() when started != null:
return started(_that.totalFiles,_that.totalBytes);case DownloadGameProgress_FileSizeVerification() when fileSizeVerification != null:
return fileSizeVerification(_that.checkedFiles);case DownloadGameProgress_FileAllocationStarted() when fileAllocationStarted != null:
return fileAllocationStarted(_that.totalFiles,_that.totalBytes);case DownloadGameProgress_FileAllocation() when fileAllocation != null:
return fileAllocation(_that.allocatedFiles,_that.allocatedBytes);case DownloadGameProgress_AllocationError() when allocationError != null:
return allocationError(_that.field0);case DownloadGameProgress_DownloadProgress() when downloadProgress != null:
return downloadProgress(_that.downloadedBytes);case DownloadGameProgress_Finished() when finished != null:
return finished();case DownloadGameProgress_Cancelled() when cancelled != null:
return cancelled();case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt totalFiles,  BigInt totalBytes)  started,required TResult Function( BigInt checkedFiles)  fileSizeVerification,required TResult Function( BigInt totalFiles,  BigInt totalBytes)  fileAllocationStarted,required TResult Function( BigInt allocatedFiles,  BigInt allocatedBytes)  fileAllocation,required TResult Function( String field0)  allocationError,required TResult Function( BigInt downloadedBytes)  downloadProgress,required TResult Function()  finished,required TResult Function()  cancelled,}) {final _that = this;
switch (_that) {
case DownloadGameProgress_Started():
return started(_that.totalFiles,_that.totalBytes);case DownloadGameProgress_FileSizeVerification():
return fileSizeVerification(_that.checkedFiles);case DownloadGameProgress_FileAllocationStarted():
return fileAllocationStarted(_that.totalFiles,_that.totalBytes);case DownloadGameProgress_FileAllocation():
return fileAllocation(_that.allocatedFiles,_that.allocatedBytes);case DownloadGameProgress_AllocationError():
return allocationError(_that.field0);case DownloadGameProgress_DownloadProgress():
return downloadProgress(_that.downloadedBytes);case DownloadGameProgress_Finished():
return finished();case DownloadGameProgress_Cancelled():
return cancelled();}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt totalFiles,  BigInt totalBytes)?  started,TResult? Function( BigInt checkedFiles)?  fileSizeVerification,TResult? Function( BigInt totalFiles,  BigInt totalBytes)?  fileAllocationStarted,TResult? Function( BigInt allocatedFiles,  BigInt allocatedBytes)?  fileAllocation,TResult? Function( String field0)?  allocationError,TResult? Function( BigInt downloadedBytes)?  downloadProgress,TResult? Function()?  finished,TResult? Function()?  cancelled,}) {final _that = this;
switch (_that) {
case DownloadGameProgress_Started() when started != null:
return started(_that.totalFiles,_that.totalBytes);case DownloadGameProgress_FileSizeVerification() when fileSizeVerification != null:
return fileSizeVerification(_that.checkedFiles);case DownloadGameProgress_FileAllocationStarted() when fileAllocationStarted != null:
return fileAllocationStarted(_that.totalFiles,_that.totalBytes);case DownloadGameProgress_FileAllocation() when fileAllocation != null:
return fileAllocation(_that.allocatedFiles,_that.allocatedBytes);case DownloadGameProgress_AllocationError() when allocationError != null:
return allocationError(_that.field0);case DownloadGameProgress_DownloadProgress() when downloadProgress != null:
return downloadProgress(_that.downloadedBytes);case DownloadGameProgress_Finished() when finished != null:
return finished();case DownloadGameProgress_Cancelled() when cancelled != null:
return cancelled();case _:
  return null;

}
}

}

/// @nodoc


class DownloadGameProgress_Started extends DownloadGameProgress {
  const DownloadGameProgress_Started({required this.totalFiles, required this.totalBytes}): super._();
  

 final  BigInt totalFiles;
 final  BigInt totalBytes;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadGameProgress_StartedCopyWith<DownloadGameProgress_Started> get copyWith => _$DownloadGameProgress_StartedCopyWithImpl<DownloadGameProgress_Started>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress_Started&&(identical(other.totalFiles, totalFiles) || other.totalFiles == totalFiles)&&(identical(other.totalBytes, totalBytes) || other.totalBytes == totalBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,totalFiles,totalBytes);
}

@override
String toString() {
    return 'DownloadGameProgress.started(totalFiles: $totalFiles, totalBytes: $totalBytes)';
}


}

/// @nodoc
abstract mixin class $DownloadGameProgress_StartedCopyWith<$Res> implements $DownloadGameProgressCopyWith<$Res> {
  factory $DownloadGameProgress_StartedCopyWith(DownloadGameProgress_Started value, $Res Function(DownloadGameProgress_Started) _then) = _$DownloadGameProgress_StartedCopyWithImpl;
@useResult
$Res call({
 BigInt totalFiles, BigInt totalBytes
});




}
/// @nodoc
class _$DownloadGameProgress_StartedCopyWithImpl<$Res>
    implements $DownloadGameProgress_StartedCopyWith<$Res> {
  _$DownloadGameProgress_StartedCopyWithImpl(this._self, this._then);

  final DownloadGameProgress_Started _self;
  final $Res Function(DownloadGameProgress_Started) _then;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? totalFiles = null,Object? totalBytes = null,}) {
  return _then(DownloadGameProgress_Started(
totalFiles: null == totalFiles ? _self.totalFiles : totalFiles // ignore: cast_nullable_to_non_nullable
as BigInt,totalBytes: null == totalBytes ? _self.totalBytes : totalBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class DownloadGameProgress_FileSizeVerification extends DownloadGameProgress {
  const DownloadGameProgress_FileSizeVerification({required this.checkedFiles}): super._();
  

 final  BigInt checkedFiles;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadGameProgress_FileSizeVerificationCopyWith<DownloadGameProgress_FileSizeVerification> get copyWith => _$DownloadGameProgress_FileSizeVerificationCopyWithImpl<DownloadGameProgress_FileSizeVerification>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress_FileSizeVerification&&(identical(other.checkedFiles, checkedFiles) || other.checkedFiles == checkedFiles));
}


@override
int get hashCode {
    return Object.hash(runtimeType,checkedFiles);
}

@override
String toString() {
    return 'DownloadGameProgress.fileSizeVerification(checkedFiles: $checkedFiles)';
}


}

/// @nodoc
abstract mixin class $DownloadGameProgress_FileSizeVerificationCopyWith<$Res> implements $DownloadGameProgressCopyWith<$Res> {
  factory $DownloadGameProgress_FileSizeVerificationCopyWith(DownloadGameProgress_FileSizeVerification value, $Res Function(DownloadGameProgress_FileSizeVerification) _then) = _$DownloadGameProgress_FileSizeVerificationCopyWithImpl;
@useResult
$Res call({
 BigInt checkedFiles
});




}
/// @nodoc
class _$DownloadGameProgress_FileSizeVerificationCopyWithImpl<$Res>
    implements $DownloadGameProgress_FileSizeVerificationCopyWith<$Res> {
  _$DownloadGameProgress_FileSizeVerificationCopyWithImpl(this._self, this._then);

  final DownloadGameProgress_FileSizeVerification _self;
  final $Res Function(DownloadGameProgress_FileSizeVerification) _then;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? checkedFiles = null,}) {
  return _then(DownloadGameProgress_FileSizeVerification(
checkedFiles: null == checkedFiles ? _self.checkedFiles : checkedFiles // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class DownloadGameProgress_FileAllocationStarted extends DownloadGameProgress {
  const DownloadGameProgress_FileAllocationStarted({required this.totalFiles, required this.totalBytes}): super._();
  

 final  BigInt totalFiles;
 final  BigInt totalBytes;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadGameProgress_FileAllocationStartedCopyWith<DownloadGameProgress_FileAllocationStarted> get copyWith => _$DownloadGameProgress_FileAllocationStartedCopyWithImpl<DownloadGameProgress_FileAllocationStarted>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress_FileAllocationStarted&&(identical(other.totalFiles, totalFiles) || other.totalFiles == totalFiles)&&(identical(other.totalBytes, totalBytes) || other.totalBytes == totalBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,totalFiles,totalBytes);
}

@override
String toString() {
    return 'DownloadGameProgress.fileAllocationStarted(totalFiles: $totalFiles, totalBytes: $totalBytes)';
}


}

/// @nodoc
abstract mixin class $DownloadGameProgress_FileAllocationStartedCopyWith<$Res> implements $DownloadGameProgressCopyWith<$Res> {
  factory $DownloadGameProgress_FileAllocationStartedCopyWith(DownloadGameProgress_FileAllocationStarted value, $Res Function(DownloadGameProgress_FileAllocationStarted) _then) = _$DownloadGameProgress_FileAllocationStartedCopyWithImpl;
@useResult
$Res call({
 BigInt totalFiles, BigInt totalBytes
});




}
/// @nodoc
class _$DownloadGameProgress_FileAllocationStartedCopyWithImpl<$Res>
    implements $DownloadGameProgress_FileAllocationStartedCopyWith<$Res> {
  _$DownloadGameProgress_FileAllocationStartedCopyWithImpl(this._self, this._then);

  final DownloadGameProgress_FileAllocationStarted _self;
  final $Res Function(DownloadGameProgress_FileAllocationStarted) _then;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? totalFiles = null,Object? totalBytes = null,}) {
  return _then(DownloadGameProgress_FileAllocationStarted(
totalFiles: null == totalFiles ? _self.totalFiles : totalFiles // ignore: cast_nullable_to_non_nullable
as BigInt,totalBytes: null == totalBytes ? _self.totalBytes : totalBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class DownloadGameProgress_FileAllocation extends DownloadGameProgress {
  const DownloadGameProgress_FileAllocation({required this.allocatedFiles, required this.allocatedBytes}): super._();
  

 final  BigInt allocatedFiles;
 final  BigInt allocatedBytes;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadGameProgress_FileAllocationCopyWith<DownloadGameProgress_FileAllocation> get copyWith => _$DownloadGameProgress_FileAllocationCopyWithImpl<DownloadGameProgress_FileAllocation>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress_FileAllocation&&(identical(other.allocatedFiles, allocatedFiles) || other.allocatedFiles == allocatedFiles)&&(identical(other.allocatedBytes, allocatedBytes) || other.allocatedBytes == allocatedBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,allocatedFiles,allocatedBytes);
}

@override
String toString() {
    return 'DownloadGameProgress.fileAllocation(allocatedFiles: $allocatedFiles, allocatedBytes: $allocatedBytes)';
}


}

/// @nodoc
abstract mixin class $DownloadGameProgress_FileAllocationCopyWith<$Res> implements $DownloadGameProgressCopyWith<$Res> {
  factory $DownloadGameProgress_FileAllocationCopyWith(DownloadGameProgress_FileAllocation value, $Res Function(DownloadGameProgress_FileAllocation) _then) = _$DownloadGameProgress_FileAllocationCopyWithImpl;
@useResult
$Res call({
 BigInt allocatedFiles, BigInt allocatedBytes
});




}
/// @nodoc
class _$DownloadGameProgress_FileAllocationCopyWithImpl<$Res>
    implements $DownloadGameProgress_FileAllocationCopyWith<$Res> {
  _$DownloadGameProgress_FileAllocationCopyWithImpl(this._self, this._then);

  final DownloadGameProgress_FileAllocation _self;
  final $Res Function(DownloadGameProgress_FileAllocation) _then;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? allocatedFiles = null,Object? allocatedBytes = null,}) {
  return _then(DownloadGameProgress_FileAllocation(
allocatedFiles: null == allocatedFiles ? _self.allocatedFiles : allocatedFiles // ignore: cast_nullable_to_non_nullable
as BigInt,allocatedBytes: null == allocatedBytes ? _self.allocatedBytes : allocatedBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class DownloadGameProgress_AllocationError extends DownloadGameProgress {
  const DownloadGameProgress_AllocationError(this.field0): super._();
  

 final  String field0;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadGameProgress_AllocationErrorCopyWith<DownloadGameProgress_AllocationError> get copyWith => _$DownloadGameProgress_AllocationErrorCopyWithImpl<DownloadGameProgress_AllocationError>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress_AllocationError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0);
}

@override
String toString() {
    return 'DownloadGameProgress.allocationError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $DownloadGameProgress_AllocationErrorCopyWith<$Res> implements $DownloadGameProgressCopyWith<$Res> {
  factory $DownloadGameProgress_AllocationErrorCopyWith(DownloadGameProgress_AllocationError value, $Res Function(DownloadGameProgress_AllocationError) _then) = _$DownloadGameProgress_AllocationErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$DownloadGameProgress_AllocationErrorCopyWithImpl<$Res>
    implements $DownloadGameProgress_AllocationErrorCopyWith<$Res> {
  _$DownloadGameProgress_AllocationErrorCopyWithImpl(this._self, this._then);

  final DownloadGameProgress_AllocationError _self;
  final $Res Function(DownloadGameProgress_AllocationError) _then;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(DownloadGameProgress_AllocationError(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class DownloadGameProgress_DownloadProgress extends DownloadGameProgress {
  const DownloadGameProgress_DownloadProgress({required this.downloadedBytes}): super._();
  

 final  BigInt downloadedBytes;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadGameProgress_DownloadProgressCopyWith<DownloadGameProgress_DownloadProgress> get copyWith => _$DownloadGameProgress_DownloadProgressCopyWithImpl<DownloadGameProgress_DownloadProgress>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress_DownloadProgress&&(identical(other.downloadedBytes, downloadedBytes) || other.downloadedBytes == downloadedBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,downloadedBytes);
}

@override
String toString() {
    return 'DownloadGameProgress.downloadProgress(downloadedBytes: $downloadedBytes)';
}


}

/// @nodoc
abstract mixin class $DownloadGameProgress_DownloadProgressCopyWith<$Res> implements $DownloadGameProgressCopyWith<$Res> {
  factory $DownloadGameProgress_DownloadProgressCopyWith(DownloadGameProgress_DownloadProgress value, $Res Function(DownloadGameProgress_DownloadProgress) _then) = _$DownloadGameProgress_DownloadProgressCopyWithImpl;
@useResult
$Res call({
 BigInt downloadedBytes
});




}
/// @nodoc
class _$DownloadGameProgress_DownloadProgressCopyWithImpl<$Res>
    implements $DownloadGameProgress_DownloadProgressCopyWith<$Res> {
  _$DownloadGameProgress_DownloadProgressCopyWithImpl(this._self, this._then);

  final DownloadGameProgress_DownloadProgress _self;
  final $Res Function(DownloadGameProgress_DownloadProgress) _then;

/// Create a copy of DownloadGameProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? downloadedBytes = null,}) {
  return _then(DownloadGameProgress_DownloadProgress(
downloadedBytes: null == downloadedBytes ? _self.downloadedBytes : downloadedBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class DownloadGameProgress_Finished extends DownloadGameProgress {
  const DownloadGameProgress_Finished(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress_Finished);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'DownloadGameProgress.finished()';
}


}




/// @nodoc


class DownloadGameProgress_Cancelled extends DownloadGameProgress {
  const DownloadGameProgress_Cancelled(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadGameProgress_Cancelled);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'DownloadGameProgress.cancelled()';
}


}




// dart format on
